// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

// 区块链引擎，支持非即时的BTF权限
//! A blockchain engine that supports a non-instant BFT proof-of-authority.


// We will run a chain with Authority Round consensus engine. First we need to create a basic chain spec with all required fields.
// 使用Authority Round consensus engine创建一条新的链，首先要创建一个创世块
// validators是验证器列表，一组允许参与共识的账户，可以在创世块中指定

// 一些可选参数：
// "blockReward" 区块奖励
// "validateScoreTransition" Optional, will be included for block 0 by default - Block after which a block’s difficulty is verified.
// "validateStepTransition" Block after which a double block proposing - e.g when parent and current block are equal - is invalid and considered as a malicious behavior.
// "immediateTransitions" - bool - Determines whether the validator set transition is applied immediately without waiting for finality (true) or not (false).
// "blockRewardContractTransition" Block at which the block reward contract should start being used.
// "blockRewardContractAddress" Block reward contract address, setting the block reward contract. This option overrides the static block reward definition.
// "maximumUncleCountTransition" Block at which maximum uncle count should be considered.
// "maximumUncleCount" Maximum number of accepted uncles.
// "emptyStepsTransition" Block at which empty step messages should start.
// "maximumEmptySteps" Maximum number of accepted empty steps. 最大可接受的空步数

// {
//     "name": "DemoPoA",
//     "engine": {
//         "authorityRound": {
//             "params": {
//                 "stepDuration": "5", //这是出块时间，设置为5秒
//                 "validators" : { //验证人，暂时是空的
//                     "list": []
//                 }
//             }
//         }
//     },
//     "params": {
//         "gasLimitBoundDivisor": "0x400",  //gas调整值
//         "maximumExtraDataSize": "0x20",
//         "minGasLimit": "0x1388",
//         "networkID" : "0x2323",
//         "eip155Transition": 0,
//         "validateChainIdTransition": 0,
//         "eip140Transition": 0,
//         "eip211Transition": 0,
//         "eip214Transition": 0,
//         "eip658Transition": 0
//     },

// Authority Round consensus的其他字段值

// The genesis seal should not be changed unless a hard fork is conducted.
// If malicious authorities are possible then --force-sealing is advised, this will ensure that the correct chain is the longest (making it BFT with finality of authorities_count * step_duration given no network partitions).

//     "genesis": {
//         "seal": { // 出块
//             "authorityRound": {
//                 "step": "0x0",//区块高度？
//                 "signature": "0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
//             }
//         },
			// 难度
//         "difficulty": "0x20000",
			// gas限制
//         "gasLimit": "0x5B8D80"
//     },
// 包含标准的以太坊预置合约
//     "accounts": {
//         "0x0000000000000000000000000000000000000001": { "balance": "1", "builtin": { "name": "ecrecover", "pricing": { "linear": { "base": 3000, "word": 0 } } } },
//         "0x0000000000000000000000000000000000000002": { "balance": "1", "builtin": { "name": "sha256", "pricing": { "linear": { "base": 60, "word": 12 } } } },
//         "0x0000000000000000000000000000000000000003": { "balance": "1", "builtin": { "name": "ripemd160", "pricing": { "linear": { "base": 600, "word": 120 } } } },
//         "0x0000000000000000000000000000000000000004": { "balance": "1", "builtin": { "name": "identity", "pricing": { "linear": { "base": 15, "word": 3 } } } }
//     }
// }

use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::iter::FromIterator;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering as AtomicOrdering};
use std::sync::{Weak, Arc};
use std::time::{UNIX_EPOCH, SystemTime, Duration};

use account_provider::AccountProvider;
use block::*;
use client::EngineClient;
use engines::{Engine, Seal, EngineError, ConstructedVerifier};
use engines::block_reward;
use engines::block_reward::{BlockRewardContract, RewardKind};
use error::{Error, ErrorKind, BlockError};
use ethjson;
use machine::{AuxiliaryData, Call, EthereumMachine};
use hash::keccak;
use header::{Header, BlockNumber, ExtendedHeader};
use super::signer::EngineSigner;
use super::validator_set::{ValidatorSet, SimpleList, new_validator_set};
use self::finality::RollingFinality;
use ethkey::{self, Password, Signature};
use io::{IoContext, IoHandler, TimerToken, IoService};
use itertools::{self, Itertools};

// TODO: 373行RlpStream
// 以太坊中的RLP(递归长度前缀)编码
// 是以太坊中数据序列化/反序列化的主要方法，区块、交易等数据结构在 网络传输和持久化 时会先经过RLP编码后再存储到数据库中。
use rlp::{encode, Decodable, DecoderError, Encodable, RlpStream, Rlp};
use ethereum_types::{H256, H520, Address, U128, U256};
use parking_lot::{Mutex, RwLock};
use unexpected::{Mismatch, OutOfBounds};

// epoch
// 确定性
mod finality;

// 将参数定义在结构体中，这样代码更明确
// 任何Authority Round相关的字段都可以到这里来找
// 可配置

// "Seal a block"是一个在私有链中描述""mine a block"的术语，其实就是打包一个区块
// "Seal a block" is a proposed term to describe "mine a block" in a private chain.

// TODO: 结构体 AuthorityRoundParams
// "AuthorityRound"参数
/// `AuthorityRound` params.
pub struct AuthorityRoundParams {
	/// Time to wait before next block or authority switching,
	/// in seconds.
	///
	/// Deliberately typed as u16 as too high of a value leads
	/// to slow block issuance.
	// 这里值大于u16最大值，会导致出块速度减慢
	// 出块时间间隔
	pub step_duration: u16,
	/// 开始处块,
	pub start_step: Option<u64>,
	/// Valid validators.
	// 验证人
	pub validators: Box<ValidatorSet>,
	/// Chain score validation transition block.
	// 权重
	pub validate_score_transition: u64,
	/// Monotonic step validation transition block.
	// step权重，比如A:B:C = 1:2:1 那么step就是time / duration % 4，B节点可以出块两次
	pub validate_step_transition: u64,
	/// Immediate transitions.
	pub immediate_transitions: bool,
	/// Block reward in base units.
	// 出块奖励
	pub block_reward: U256,
	/// Block reward contract transition block.
	pub block_reward_contract_transition: u64,
	/// Block reward contract.
	// 区块奖励合约
	pub block_reward_contract: Option<BlockRewardContract>,
	/// Number of accepted uncles transition block.
	pub maximum_uncle_count_transition: u64,
	/// Number of accepted uncles.
	pub maximum_uncle_count: usize,
	/// Empty step messages transition block.
	pub empty_steps_transition: u64,
	/// Number of accepted empty steps.
	pub maximum_empty_steps: usize,
}

const U16_MAX: usize = ::std::u16::MAX as usize;
  
// 为结构体 AuthorityRoundParams 实现trait
impl From<ethjson::spec::AuthorityRoundParams> for AuthorityRoundParams {

	// from返回一个具有基本设置的实例
	fn from(p: ethjson::spec::AuthorityRoundParams) -> Self {
		let mut step_duration_usize: usize = p.step_duration.into();

		// 如果设置的step_duration_usize过大，将其重置为u16所能存储的最大值
		if step_duration_usize > U16_MAX {
			step_duration_usize = U16_MAX;
			warn!(target: "engine", "step_duration is too high ({}), setting it to {}", step_duration_usize, U16_MAX);
		}

		AuthorityRoundParams {
			// step设置
			step_duration: step_duration_usize as u16,
			validators: new_validator_set(p.validators),
			// 迭代器适配器map
			start_step: p.start_step.map(Into::into),

			// 验证人过渡
			validate_score_transition: p.validate_score_transition.map_or(0, Into::into),
			validate_step_transition: p.validate_step_transition.map_or(0, Into::into),
			immediate_transitions: p.immediate_transitions.unwrap_or(false),

			// 区块奖励
			block_reward: p.block_reward.map_or_else(Default::default, Into::into),
			block_reward_contract_transition: p.block_reward_contract_transition.map_or(0, Into::into),
			block_reward_contract: match (p.block_reward_contract_code, p.block_reward_contract_address) {
				(Some(code), _) => Some(BlockRewardContract::new_from_code(Arc::new(code.into()))),
				(_, Some(address)) => Some(BlockRewardContract::new_from_address(address.into())),
				(None, None) => None,
			},

			// 叔块
			maximum_uncle_count_transition: p.maximum_uncle_count_transition.map_or(0, Into::into),
			maximum_uncle_count: p.maximum_uncle_count.map_or(0, Into::into),
			empty_steps_transition: p.empty_steps_transition.map_or(u64::max_value(), |n| ::std::cmp::max(n.into(), 1)),
			maximum_empty_steps: p.maximum_empty_steps.map_or(0, Into::into),
		}
	}
}


// ------------------------------------------------------Step确定出块顺序，保证同步、一致性
// step以usize类型储存
// 载入、剩余时间、增量、校准、验证
// 载入：使用AtomicUsize类型，调用store方法，将step存储在里面
// 剩余时间：match expected_seconds来判断上一个step结束、_、None集中状态 TODO: rust match _
// 增量：在AtomicOrdering::SeqCst中存储一个step，并判断是否超出usize最大值
// 验证：节点之间会有时间出入，确保节点之间的时间差异不会很大，比如有三个节点：
	// A：step = 100 / 10
	// B：step = 113 / 10
	// C：step = 300 / 10
	// AtomicOrdering::SeqCst存储的step = 100 / 10
	// 那么A节点的step就是正常的，B节点是可以等待的，C节点的step应该直接被拒绝
	// C节点这里不是直接被拒绝，当发现C给我传入的step过大，直接调用calibrate校准方法，使用self计算出new_step，将new_step存储起来
	// 然后load最新的step赋值给变量current

// Helper for managing the step.
// TODO: 结构体 Step
// step辅助管理
#[derive(Debug)]
struct Step {
	// 是否校准
	calibrate: bool, // whether calibration is enabled.
	// AtomicUsize是可以再线程之间安全共享的整数类型，这里用来对step做store、load、fetch_add等操作
	inner: AtomicUsize,
	// 时间间隔
	duration: u16,
}

// 为结构体实现的方法
impl Step {

	// 从AtomicOrdering::SeqCst里面load 最新的step初始设置
	fn load(&self) -> usize { self.inner.load(AtomicOrdering::SeqCst) }

	// 剩余时间，距离下一次出块的时间
	fn duration_remaining(&self) -> Duration {
		// 先获取当前时间
		let now = unix_now();

		let expected_seconds = (self.load() as u64)
			.checked_add(1)
			// 闭包
			.and_then(|ctr| ctr.checked_mul(self.duration as u64))
			.map(Duration::from_secs);

		match expected_seconds {
			Some(step_end) if step_end > now => step_end - now,
			Some(_) => Duration::from_secs(0),
			None => {
				let ctr = self.load();
				error!(target: "engine", "Step counter is too high: {}, aborting", ctr);
				panic!("step counter is too high: {}", ctr)
			},
		}

	}

// 增量
	fn increment(&self) {
		use std::usize;
		// fetch_add won't panic on overflow but will rather wrap
		// around, leading to zero as the step counter, which might
		// lead to unexpected situations, so it's better to shut down.
		if self.inner.fetch_add(1, AtomicOrdering::SeqCst) == usize::MAX {
			error!(target: "engine", "Step counter is too high: {}, aborting", usize::MAX);
			panic!("step counter is too high: {}", usize::MAX);
		}
	}

// 校准
	fn calibrate(&self) {
		if self.calibrate {
			// 新的step = 当前时间秒数 / 出块时间
			let new_step = unix_now().as_secs() / (self.duration as u64);
			// store-存储一个值到Atomic整数中，使用order描述此操作的内存顺序
			// 将new_step作为usize类型存储到AtomicOrdering::SeqCst中
			self.inner.store(new_step as usize, AtomicOrdering::SeqCst);
		}
	}

// 最后做校验，验证
// given传入的就是一个usize类型的step
	fn check_future(&self, given: usize) -> Result<(), Option<OutOfBounds<u64>>> {

		// 相当于是最大偏离值，允许的偏移量
		const REJECTED_STEP_DRIFT: usize = 4;

		// Verify if the step is correct.
		// 如果当前step是正确的，直接返回OK 
		if given <= self.load() {
			return Ok(());
		}

		// Make absolutely sure that the given step is incorrect.
		// 如果一个step不正确，通过self获取到当前的step
		self.calibrate();
		let current = self.load();

		// reject blocks too far in the future
		// step偏差太大了，被拒绝
		if given > current + REJECTED_STEP_DRIFT {
			Err(None)
		// wait a bit for blocks in near future
		// 等待
		} else if given > current {
			let d = self.duration as u64;
			Err(Some(OutOfBounds {
				min: None,
				max: Some(d * current as u64),
				found: d * given as u64,
			}))
		} else {
			Ok(())
		}
	}
}

// -----------------------------------------------------------权重
// 权重计算,调整难度
// Chain scorin total weight is sqrt(U256::max_value())*height - step
// 链的得分，每条链都有一个得分，正常的节点会在得分最高的那条链上打包
// Chain scoring: total weight is sqrt(U256::max_value())*height - step
fn calculate_score(parent_step: U256, current_step: U256, current_empty_steps: U256) -> U256 {
	U256::from(U128::max_value()) + parent_step - current_step + current_empty_steps
}

// TODO: 结构体 EpochManager
// 确定性相关的，更多区块组成的时间概念
// TODO: epoch
// -----------------------------------------------------------Epoch
// 一定数量的区块可以称为一个epoch，是在区块链上的一个时间概念，epoch之间会有一个过渡的问题
// 以太坊挖矿需要额外的资源来挖掘：内存，使用GPU挖掘时，这意味这显卡上的内存，随着时间的推移，以太坊故意使采矿更加耗费内存，每隔固定的时间就调整一次。每30000个块就会有一个新的数据(DAG)用于挖掘新块。
// 每个新的30000个块被称作epoch，epoch开关是在下载DAG文件的时候加载的。所以就涉及到epoch之间的过渡问题
// 所以下面的结构体包括的就是epoch的hash、epoch的过渡数(应该指的是从上个epoch过渡之后又生成了多少区块)、确定性检查、是否强制过渡？
// Epoch在以太坊中默认为30000，区块高度为3w的整数倍时，到达Epoch时间点
struct EpochManager {
	// 过渡hash
	epoch_transition_hash: H256,
	epoch_transition_number: BlockNumber,
	// 确定性检查
	finality_checker: RollingFinality,
	// 是否强制(这里应该是是否强制进行确定性检查)
	force: bool,
}

impl EpochManager {

	// 一个空的epoch，参数都是默认值，区块数为0，相当于是一个还未开始的epoch
	fn blank() -> Self {
		EpochManager {
			epoch_transition_hash: H256::default(),
			epoch_transition_number: 0,
			finality_checker: RollingFinality::blank(Vec::new()),
			force: true,
		}
	}


// TODO:2.zoom_to
// 根据给定的header调整epoch，如果调整成功返回true
	// zoom to epoch for given header. returns true if succeeded, false otherwise.
	// 缩放到给定区块头的epoch
	fn zoom_to(&mut self, client: &EngineClient, machine: &EthereumMachine, validators: &ValidatorSet, header: &Header) -> bool {

		// 最后的块是否是父块 
		// TODO:
		// subchain_head() 
		// parent_hash() 
		let last_was_parent = self.finality_checker.subchain_head() == Some(header.parent_hash().clone());

		// early exit for current target == chain head, but only if the epochs are 
		// the same.
		if last_was_parent && !self.force { 
			return true; 
		} 

		self.force = false;
		debug!(target: "engine", "Zooming to epoch for block {}", header.hash());

		// epoch_transition_for can be an expensive call, but in the absence of
		// forks it will only need to be called for the block directly after
		// epoch transition, in which case it will be O(1) and require a single
		// DB lookup.
		// 
		let last_transition = match client.epoch_transition_for(*header.parent_hash()) {
			Some(t) => t,
			None => {
				// this really should never happen unless the block passed
				// hasn't got a parent in the database.
				debug!(target: "engine", "No genesis transition found.");
				return false;
			}
		};

		// extract other epoch set if it's not the same as the last.
		// 如果他与上一个不同，则提取其他的epoch set
		if last_transition.block_hash != self.epoch_transition_hash {
			let (signal_number, set_proof, _) = destructure_proofs(&last_transition.proof)
				.expect("proof produced by this engine; therefore it is valid; qed");

			trace!(target: "engine", "extracting epoch set for epoch ({}, {}) signalled at #{}",
				last_transitifinality_checkeron.block_number, last_transition.block_hash, signal_number);

			let first = signal_number == 0;
			let epoch_set = validators.epoch_set(
				first,
				machine,
				signal_number, // use signal number so multi-set first calculation is correct.
				set_proof,
			)
				.ok()
				.map(|(list, _)| list.into_inner())
				.expect("proof produced by this engine; therefore it is valid; qed");

			self.finality_checker = RollingFinality::blank(epoch_set);
		}

		self.epoch_transition_hash = last_transition.block_hash;
		self.epoch_transition_number = last_transition.block_number;

		true
	}

	// note new epoch hash. this will force the next block to re-load
	// the epoch set
	// TODO: optimize and don't require re-loading after epoch change.
	// 注意新epoch的hash，这会强制下一个块重新加载epoch set
	fn note_new_epoch(&mut self) {
		self.force = true;
	}

	/// Get validator set. Zoom to the correct epoch first.
	// 获取epoch中的验证者们
	fn validators(&self) -> &SimpleList {
		self.finality_checker.validators()
	}
}


// 出块确定性，共识
// TODO:关于Step：EmptyStep、SealedEmptyStep、PermissionedStep
// TODO:空消息、空消息打包到块、

// 轮到出块时却没有交易，其他验证者积累这些信息，然后将他们作为证据包含在proof中

// TODO:Seal是做什么用的
// 区块头，封装的，打包
// 状态的根，合约里每个值，验证交易，交易执行-上一个区块状态根-当前区块根
// 另一个是event日志的根
// 1，状态根(组织，状态回滚，状态树M默克尔树P字典排序T)
// TODO:区块信息
/// A message broadcast by authorities when it's their turn to seal a block but there are no
/// transactions. Other authorities accumulate these messages and later include them in the seal as
/// proof.
/// 
// TODO: 结构体 EmptyStep，这里面的EmptyStep是做什么用的,还有下面的SealedEmptyStep

// ----------------------------------------------------------------------------emptyStep
// 实现了空Step的一些方法，一个EmptyStep包含一个签名，一个step和一个父区块hash
#[derive(Clone, Debug)]
struct EmptyStep {
	//H520、H256 Unformatted binary data of fixed length. 来自ethereum-types
	signature: H520,
	// step
	step: usize,
	// 父hsah
	parent_hash: H256,
}

// 当轮到节点打包的时候，发现这期间只有一些消息，但是没有任何交易，他们就会验证这些消息，然后广播，会一直积累这些消息，直到下一个包含交易的区块生成的时候，将这些消息一并打包进去，然后节点们获得相应的奖励
// SealedEmptyStep就是将这些消息积累起来

impl EmptyStep {

	// 使用sealed_empty_step和parent_hash返回一个EmptyStep
	// 打包一个EmptyStep
	// EmptyStep相比SealedEmptyStep多了parent_hash
	fn from_sealed(sealed_empty_step: SealedEmptyStep, parent_hash: &H256) -> EmptyStep {
		let signature = sealed_empty_step.signature;
		let step = sealed_empty_step.step;
		let parent_hash = parent_hash.clone();
		EmptyStep { signature, step, parent_hash }
	}

// 验证
// 这里验证的是一些验证者和一个EmptyStep，返回bool类型
// 目的：验证这个EmptyStep是否是这些节点验证人签名的
	fn verify(&self, validators: &ValidatorSet) -> Result<bool, Error> {
		let message = keccak(empty_step_rlp(self.step, &self.parent_hash));
		let correct_proposer = step_proposer(validators, &self.parent_hash, self.step);

		// 调用ethkey模块中的verify_address接口
		ethkey::verify_address(&correct_proposer, &self.signature.into(), &message)
			.map_err(|e| e.into())
	}


// 通过签名和消息还原公钥，再通过公钥返回地址
	fn author(&self) -> Result<Address, Error> {
		let message = keccak(empty_step_rlp(self.step, &self.parent_hash));
		// 调用recover接口
		let public = ethkey::recover(&self.signature.into(), &message)?;
		Ok(ethkey::public_to_address(&public))
	}

// 传入EmptyStep，得到他的签名和step，返回一个sealedEmptyStep
// 目的：将EmptyStep打包
	fn sealed(&self) -> SealedEmptyStep {
		let signature = self.signature;
		let step = self.step;
		SealedEmptyStep { signature, step }
	}
}

// 为结构体实现trait
impl fmt::Display for EmptyStep {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "({}, {}, {})", self.signature, self.step, self.parent_hash)
	}
}

// RLP编码
impl Encodable for EmptyStep {
	fn rlp_append(&self, s: &mut RlpStream) {
		let empty_step_rlp = empty_step_rlp(self.step, &self.parent_hash);
		s.begin_list(2)
			.append(&self.signature)
			.append_raw(&empty_step_rlp, 1);
	}
}

// RLP解码
impl Decodable for EmptyStep {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		let signature = rlp.val_at(0)?;
		let empty_step_rlp = rlp.at(1)?;

		let step = empty_step_rlp.val_at(0)?;
		let parent_hash = empty_step_rlp.val_at(1)?;

		Ok(EmptyStep { signature, step, parent_hash })
	}
}


// 两个编解码相关的方法
// 这个是将全部的empty_step输出为u8类型的Vec
pub fn empty_step_full_rlp(signature: &H520, empty_step_rlp: &[u8]) -> Vec<u8> {
	let mut s = RlpStream::new_list(2);
	s.append(signature).append_raw(empty_step_rlp, 1);
	s.out()
}

// 这个是将一个empty_step输出为u8类型的Vec
pub fn empty_step_rlp(step: usize, parent_hash: &H256) -> Vec<u8> {
	let mut s = RlpStream::new_list(2);
	s.append(&step).append(parent_hash);
	s.out()
}

/// An empty step message that is included in a seal, the only difference is that it doesn't include
/// the `parent_hash` in order to save space. The included signature is of the original empty step
/// message, which can be reconstructed by using the parent hash of the block in which this sealed
/// empty message is included.
// 打包一些空step消息，唯一的区别是它没有"parent_hash"以节省空间，包含的签名是原始的空step消息，可以使用打包空step消息的区块的父hash来重建

// 将一个empty step消息打包到一个seal中，唯一的不同是为了节省空间它没有parent_hash。
// 一开始大家在轮到step的时候，发现没有交易，只有一些消息，大家就将当前step的消息签名，这个就是EmptyStep，包含一个parent_hash，做一些校验，并且节点验证人会去签名，
// 一直等等等等到一个有交易的块了，大家就将攒起来的这些EmptyStep去掉parent成为SealedEmptyStep，将其打包进这个有交易的块中。
// -------------------------------------------------------SealedEmptyStep
struct SealedEmptyStep {
	signature: H520,
	step: usize,
}

impl Encodable for SealedEmptyStep {
	fn rlp_append(&self, s: &mut RlpStream) {
		s.begin_list(2)
			.append(&self.signature)
			.append(&self.step);
	}
}

impl Decodable for SealedEmptyStep {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		let signature = rlp.val_at(0)?;
		let step = rlp.val_at(1)?;

		Ok(SealedEmptyStep { signature, step })
	}
}

// 被许可的Step，can_propose-可以发起提议
struct PermissionedStep {
	inner: Step,
	can_propose: AtomicBool,
}

/// Engine using `AuthorityRound` proof-of-authority BFT consensus.
pub struct AuthorityRound {
	transition_service: IoService<()>,
	// 这里接受一个可提议的Step
	step: Arc<PermissionedStep>,
	client: Arc<RwLock<Option<Weak<EngineClient>>>>,
	signer: RwLock<EngineSigner>,
	validators: Box<ValidatorSet>,
	validate_score_transition: u64,
	validate_step_transition: u64,
	empty_steps: Mutex<Vec<EmptyStep>>,
	epoch_manager: Mutex<EpochManager>,
	immediate_transitions: bool,
	block_reward: U256,
	block_reward_contract_transition: u64,
	block_reward_contract: Option<BlockRewardContract>,
	maximum_uncle_count_transition: u64,
	maximum_uncle_count: usize,
	empty_steps_transition: u64,
	maximum_empty_steps: usize,
	machine: EthereumMachine,
}

// header-chain validator.
// epoch验证，验证一段epoch的step、验证人、emptystep是都合法
struct EpochVerifier {
	// 被许可的Step
	step: Arc<PermissionedStep>,
	// 子链验证人
	subchain_validators: SimpleList,
	// emptystep过度
	empty_steps_transition: u64,
}

impl super::EpochVerifier<EthereumMachine> for EpochVerifier {

	// 一个简单的校验
	fn verify_light(&self, header: &Header) -> Result<(), Error> {
		// Validate the timestamp
		// 校验时间戳
		verify_timestamp(&self.step.inner, header_step(header, self.empty_steps_transition)?)?;
		// always check the seal since it's fast.
		// nothing heavier to do.
		// 因为出块速度快，所以要检查seal
		verify_external(header, &self.subchain_validators, self.empty_steps_transition)
	}

	// 确定性
	fn check_finality_proof(&self, proof: &[u8]) -> Option<Vec<H256>> {
		let mut finality_checker = RollingFinality::blank(self.subchain_validators.clone().into_inner());

		// 敲定者,应该是确定这个区块是没问题的验证人,是一个列表
		let mut finalized = Vec::new();

		let headers: Vec<Header> = Rlp::new(proof).as_list().ok()?;

		{
			let mut push_header = |parent_header: &Header, header: Option<&Header>| {
				// ensure all headers have correct number of seal fields so we can `verify_external`
				// and get `empty_steps` without panic.
				// 确保所有区块头都有正确数量的seal字段,以便于我们可以做一些验证,否则抛出异常
				if parent_header.seal().len() != header_expected_seal_fields(parent_header, self.empty_steps_transition) {
					return None
				}
				if header.iter().any(|h| h.seal().len() != header_expected_seal_fields(h, self.empty_steps_transition)) {
					return None
				}

				// `verify_external` checks that signature is correct and author == signer.
				// 检查签名是正确的,并保证author == signer
				verify_external(parent_header, &self.subchain_validators, self.empty_steps_transition).ok()?;

				//打包的验证人本身就是这个区块的一个签名人(signer) 
				let mut signers = match header {
					Some(header) => header_empty_steps_signers(header, self.empty_steps_transition).ok()?,
					_ => Vec::new(),
				};
				signers.push(parent_header.author().clone());

				let newly_finalized = finality_checker.push_hash(parent_header.hash(), signers).ok()?;
				finalized.extend(newly_finalized);

				Some(())
			};

			for window in headers.windows(2) {
				push_header(&window[0], Some(&window[1]))?;
			}

			if let Some(last) = headers.last() {
				push_header(last, None)?;
			}
		}

		if finalized.is_empty() { None } else { Some(finalized) }
	}
}

// 获取到一个有效的seal的hash
fn header_seal_hash(header: &Header, empty_steps_rlp: Option<&[u8]>) -> H256 {
	match empty_steps_rlp {
		Some(empty_steps_rlp) => {
			let mut message = header.bare_hash().to_vec();
			message.extend_from_slice(empty_steps_rlp);
			keccak(message)
		},
		None => {
			header.bare_hash()
		},
	}
}

fn header_expected_seal_fields(header: &Header, empty_steps_transition: u64) -> usize {
	if header.number() >= empty_steps_transition {
		3
	} else {
		2
	}
}

fn header_step(header: &Header, empty_steps_transition: u64) -> Result<usize, ::rlp::DecoderError> {
	let expected_seal_fields = header_expected_seal_fields(header, empty_steps_transition);
	Rlp::new(&header.seal().get(0).expect(
		&format!("was either checked with verify_block_basic or is genesis; has {} fields; qed (Make sure the spec file has a correct genesis seal)", expected_seal_fields))).as_val()
}

fn header_signature(header: &Header, empty_steps_transition: u64) -> Result<Signature, ::rlp::DecoderError> {
	let expected_seal_fields = header_expected_seal_fields(header, empty_steps_transition);
	Rlp::new(&header.seal().get(1).expect(
		&format!("was checked with verify_block_basic; has {} fields; qed", expected_seal_fields))).as_val::<H520>().map(Into::into)
}

// extracts the raw empty steps vec from the header seal. should only be called when there are 3 fields in the seal
// (i.e. header.number() >= self.empty_steps_transition)
fn header_empty_steps_raw(header: &Header) -> &[u8] {
	header.seal().get(2).expect("was checked with verify_block_basic; has 3 fields; qed")
}

// extracts the empty steps from the header seal. should only be called when there are 3 fields in the seal
// (i.e. header.number() >= self.empty_steps_transition).
fn header_empty_steps(header: &Header) -> Result<Vec<EmptyStep>, ::rlp::DecoderError> {
	let empty_steps = Rlp::new(header_empty_steps_raw(header)).as_list::<SealedEmptyStep>()?;
	Ok(empty_steps.into_iter().map(|s| EmptyStep::from_sealed(s, header.parent_hash())).collect())
}

// gets the signers of empty step messages for the given header, does not include repeated signers
fn header_empty_steps_signers(header: &Header, empty_steps_transition: u64) -> Result<Vec<Address>, Error> {
	if header.number() >= empty_steps_transition {
		let mut signers = HashSet::new();
		for empty_step in header_empty_steps(header)? {
			signers.insert(empty_step.author()?);
		}

		Ok(Vec::from_iter(signers.into_iter()))
	} else {
		Ok(Vec::new())
	}
}

// 这里是一个 step_propose， 应该是一个提议者，就是在发现没有交易只有一些消息的时候，这些验证者提议将消息签名
// 这里获取到哪一个Step的提议者地址
fn step_proposer(validators: &ValidatorSet, bh: &H256, step: usize) -> Address {
	let proposer = validators.get(bh, step);
	trace!(target: "engine", "Fetched proposer for step {}: {}", step, proposer);
	proposer
}

// 这里则是验证一个地址是否是提议者
fn is_step_proposer(validators: &ValidatorSet, bh: &H256, step: usize, address: &Address) -> bool {
	step_proposer(validators, bh, step) == *address
}

// 验证时间戳
fn verify_timestamp(step: &Step, header_step: usize) -> Result<(), BlockError> {
	match step.check_future(header_step) {
		Err(None) => {
			trace!(target: "engine", "verify_timestamp: block from the future");
			Err(BlockError::InvalidSeal.into())
		},
		Err(Some(oob)) => {
			// NOTE This error might be returned only in early stage of verification (Stage 1).
			// Returning it further won't recover the sync process.
			trace!(target: "engine", "verify_timestamp: block too early");
			let oob = oob.map(|n| SystemTime::now() + Duration::from_secs(n));
			Err(BlockError::TemporarilyInvalid(oob).into())
		},
		Ok(_) => Ok(()),
	}
}

// 外部验证，也是对决议人做一些验证
fn verify_external(header: &Header, validators: &ValidatorSet, empty_steps_transition: u64) -> Result<(), Error> {
	let header_step = header_step(header, empty_steps_transition)?;

	let proposer_signature = header_signature(header, empty_steps_transition)?;

	// 找到这个区块的提议发起人
	let correct_proposer = validators.get(header.parent_hash(), header_step);

	// 每一个区块都有一个leader或者author,也就是打包这个区块的验证人,这个角色在打包区块之后是要对当前区块发起提议的,所以首先要验证决议发起人是否是有效的
	// 如果这个区块头的author不等于这个区块的提议发起者,那这个提议者就是无效的
	let is_invalid_proposer = *header.author() != correct_proposer || {
		let empty_steps_rlp = if header.number() >= empty_steps_transition {
			Some(header_empty_steps_raw(header))
		} else {
			None
		};

		let header_seal_hash = header_seal_hash(header, empty_steps_rlp);
		!ethkey::verify_address(&correct_proposer, &proposer_signature, &header_seal_hash)?
	};

	// 最后对验证结果做出反馈
	if is_invalid_proposer {
		trace!(target: "engine", "verify_block_external: bad proposer for step: {}", header_step);
		Err(EngineError::NotProposer(Mismatch { expected: correct_proposer, found: header.author().clone() }))?
	} else {
		Ok(())
	}
}

fn combine_proofs(signal_number: BlockNumber, set_proof: &[u8], finality_proof: &[u8]) -> Vec<u8> {
	let mut stream = ::rlp::RlpStream::new_list(3);
	stream.append(&signal_number).append(&set_proof).append(&finality_proof);
	stream.out()
}

fn destructure_proofs(combined: &[u8]) -> Result<(BlockNumber, &[u8], &[u8]), Error> {
	let rlp = Rlp::new(combined);
	Ok((
		rlp.at(0)?.as_val()?,
		rlp.at(1)?.data()?,
		rlp.at(2)?.data()?,
	))
}

trait AsMillis {
	fn as_millis(&self) -> u64;
}

impl AsMillis for Duration {
	fn as_millis(&self) -> u64 {
		self.as_secs()*1_000 + (self.subsec_nanos()/1_000_000) as u64
	}
}

// A type for storing owned or borrowed data that has a common type.
// Useful for returning either a borrow or owned data from a function.
enum CowLike<'a, A: 'a + ?Sized, B> {
	Borrowed(&'a A),
	Owned(B),
}

impl<'a, A: ?Sized, B> Deref for CowLike<'a, A, B> where B: AsRef<A> {
	type Target = A;
	fn deref(&self) -> &A {
		match self {
			CowLike::Borrowed(b) => b,
			CowLike::Owned(o) => o.as_ref(),
		}
	}
}

impl AuthorityRound {
    // 创建一个AuthorityRound引擎的新的实例
	/// Create a new instance of AuthorityRound engine.
	pub fn new(our_params: AuthorityRoundParams, machine: EthereumMachine) -> Result<Arc<Self>, Error> {
		if our_params.step_duration == 0 {
			error!(target: "engine", "Authority Round step duration can't be zero, aborting");
			panic!("authority_round: step duration can't be zero")
		}
		let should_timeout = our_params.start_step.is_none();
		let initial_step = our_params.start_step.unwrap_or_else(|| (unix_now().as_secs() / (our_params.step_duration as u64))) as usize;
		let engine = Arc::new(

            // 给结构体赋值
			AuthorityRound {
				transition_service: IoService::<()>::start()?,
				step: Arc::new(PermissionedStep {
					inner: Step {
						inner: AtomicUsize::new(initial_step),
						calibrate: our_params.start_step.is_none(),
						duration: our_params.step_duration,
					},
					can_propose: AtomicBool::new(true),
				}),
				client: Arc::new(RwLock::new(None)),
				signer: Default::default(),
				validators: our_params.validators,
				validate_score_transition: our_params.validate_score_transition,
				validate_step_transition: our_params.validate_step_transition,
				empty_steps: Mutex::new(Vec::new()),
				epoch_manager: Mutex::new(EpochManager::blank()),
				immediate_transitions: our_params.immediate_transitions,
				block_reward: our_params.block_reward,
				block_reward_contract_transition: our_params.block_reward_contract_transition,
				block_reward_contract: our_params.block_reward_contract,
				maximum_uncle_count_transition: our_params.maximum_uncle_count_transition,
				maximum_uncle_count: our_params.maximum_uncle_count,
				empty_steps_transition: our_params.empty_steps_transition,
				maximum_empty_steps: our_params.maximum_empty_steps,
				machine: machine,
			});

		// Do not initialize timeouts for tests.
		if should_timeout {
			let handler = TransitionHandler {
				step: engine.step.clone(),
				client: engine.client.clone(),
			};
			engine.transition_service.register_handler(Arc::new(handler))?;
		}
		Ok(engine)
	}

	// fetch correct validator set for epoch at header, taking into account
	// finality of previous transitions.
	// 为epoch获取正确的验证者集
	fn epoch_set<'a>(&'a self, header: &Header) -> Result<(CowLike<ValidatorSet, SimpleList>, BlockNumber), Error> {
		Ok(if self.immediate_transitions {
			(CowLike::Borrowed(&*self.validators), header.number())
		} else {
			let mut epoch_manager = self.epoch_manager.lock();
			let client = match self.client.read().as_ref().and_then(|weak| weak.upgrade()) {
				Some(client) => client,
				None => {
					debug!(target: "engine", "Unable to verify sig: missing client ref.");
					return Err(EngineError::RequiresClient.into())
				}
			};

			if !epoch_manager.zoom_to(&*client, &self.machine, &*self.validators, header) {
				debug!(target: "engine", "Unable to zoom to epoch.");
				return Err(EngineError::RequiresClient.into())
			}

			(CowLike::Owned(epoch_manager.validators().clone()), epoch_manager.epoch_transition_number)
		})
	}

	fn empty_steps(&self, from_step: U256, to_step: U256, parent_hash: H256) -> Vec<EmptyStep> {
		// 使用过滤器,调用 collect 方法消费迭代器并创建一个 vector
		self.empty_steps.lock().iter().filter(|e| {
			U256::from(e.step) > from_step &&
				U256::from(e.step) < to_step &&
				e.parent_hash == parent_hash
		}).cloned().collect()
	}

// 清除以前的empty_steps
	fn clear_empty_steps(&self, step: U256) {
		// clear old `empty_steps` messages
		self.empty_steps.lock().retain(|e| U256::from(e.step) > step);
	}

	fn handle_empty_step_message(&self, empty_step: EmptyStep) {
		let mut empty_steps = self.empty_steps.lock();
		empty_steps.push(empty_step);
	}

// 生成empty_step
	fn generate_empty_step(&self, parent_hash: &H256) {
		let step = self.step.inner.load();
		let empty_step_rlp = empty_step_rlp(step, parent_hash);

		if let Ok(signature) = self.sign(keccak(&empty_step_rlp)).map(Into::into) {
			let message_rlp = empty_step_full_rlp(&signature, &empty_step_rlp);

			let parent_hash = *parent_hash;
			let empty_step = EmptyStep { signature, step, parent_hash };

			trace!(target: "engine", "broadcasting empty step message: {:?}", empty_step);
			self.broadcast_message(message_rlp);
			self.handle_empty_step_message(empty_step);

		} else {
			warn!(target: "engine", "generate_empty_step: FAIL: accounts secret key unavailable");
		}
	}

// 广播消息
	fn broadcast_message(&self, message: Vec<u8>) {
		if let Some(ref weak) = *self.client.read() {
			if let Some(c) = weak.upgrade() {
				c.broadcast_consensus_message(message);
			}
		}
	}

	fn report_skipped(&self, header: &Header, current_step: usize, parent_step: usize, validators: &ValidatorSet, set_number: u64) {
		// we're building on top of the genesis block so don't report any skipped steps
		if header.number() == 1 {
			return;
		}

		if let (true, Some(me)) = (current_step > parent_step + 1, self.signer.read().address()) {
			debug!(target: "engine", "Author {} built block with step gap. current step: {}, parent step: {}",
				   header.author(), current_step, parent_step);
			let mut reported = HashSet::new();
			for step in parent_step + 1..current_step {
				let skipped_primary = step_proposer(validators, header.parent_hash(), step);
				// Do not report this signer.
				if skipped_primary != me {
					// Stop reporting once validators start repeating.
					if !reported.insert(skipped_primary) { break; }
					self.validators.report_benign(&skipped_primary, set_number, header.number());
 				}
 			}
		}
	}
}

fn unix_now() -> Duration {
	UNIX_EPOCH.elapsed().expect("Valid time has to be set in your system.")
}

// 过渡处理
// 在transition的过程中会涉及到超时的问题
struct TransitionHandler {
	step: Arc<PermissionedStep>,
	client: Arc<RwLock<Option<Weak<EngineClient>>>>,
}

const ENGINE_TIMEOUT_TOKEN: TimerToken = 23;

// 为结构体TransitionHandler实现trait IoHandler
impl IoHandler<()> for TransitionHandler {

	// 初始化
	fn initialize(&self, io: &IoContext<()>) {
		let remaining = AsMillis::as_millis(&self.step.inner.duration_remaining());
		io.register_timer_once(ENGINE_TIMEOUT_TOKEN, Duration::from_millis(remaining))
			.unwrap_or_else(|e| warn!(target: "engine", "Failed to start consensus step timer: {}.", e))
	}

	// 超时
	fn timeout(&self, io: &IoContext<()>, timer: TimerToken) {
		if timer == ENGINE_TIMEOUT_TOKEN {
			// NOTE we might be lagging by couple of steps in case the timeout
			// has not been called fast enough.
			// Make sure to advance up to the actual step.

			// 如果超时可能就会延后几步
			while AsMillis::as_millis(&self.step.inner.duration_remaining()) == 0 {
				self.step.inner.increment();
				self.step.can_propose.store(true, AtomicOrdering::SeqCst);
				if let Some(ref weak) = *self.client.read() {
					if let Some(c) = weak.upgrade() {
						c.update_sealing();
					}
				}
			}

			// 这里应该就是前进道合适的步骤,调用register_timer_once
			let next_run_at = AsMillis::as_millis(&self.step.inner.duration_remaining()) >> 2;
			io.register_timer_once(ENGINE_TIMEOUT_TOKEN, Duration::from_millis(next_run_at))
				.unwrap_or_else(|e| warn!(target: "engine", "Failed to restart consensus step timer: {}.", e))
		}
	}
}


// 为结构体AuthorityRound实现trait Engine
// 这里应该是引擎AuthorityRound的细节实现
impl Engine<EthereumMachine> for AuthorityRound {
	fn name(&self) -> &str { "AuthorityRound" }

	fn machine(&self) -> &EthereumMachine { &self.machine }

	/// Three fields - consensus step and the corresponding proposer signature, and a list of empty
	/// step messages (which should be empty if no steps are skipped)
	/// 
	/// 一共三个字段:step、相应的提议者签名、一个空列表
	fn seal_fields(&self, header: &Header) -> usize {
		header_expected_seal_fields(header, self.empty_steps_transition)
	}

	fn step(&self) {
		self.step.inner.increment();
		self.step.can_propose.store(true, AtomicOrdering::SeqCst);
		if let Some(ref weak) = *self.client.read() {
			if let Some(c) = weak.upgrade() {
				c.update_sealing();
			}
		}
	}

// step和signature信息
	/// Additional engine-specific information for the user/developer concerning `header`.
	fn extra_info(&self, header: &Header) -> BTreeMap<String, String> {
		let step = header_step(header, self.empty_steps_transition).as_ref().map(ToString::to_string).unwrap_or("".into());
		let signature = header_signature(header, self.empty_steps_transition).as_ref().map(ToString::to_string).unwrap_or("".into());

		let mut info = map![
			"step".into() => step,
			"signature".into() => signature
		];

		if header.number() >= self.empty_steps_transition {
			let empty_steps =
				if let Ok(empty_steps) = header_empty_steps(header).as_ref() {
					format!("[{}]",
							empty_steps.iter().fold(
								"".to_string(),
								|acc, e| if acc.len() > 0 { acc + ","} else { acc } + &e.to_string()))

				} else {
					"".into()
				};

			info.insert("emptySteps".into(), empty_steps);
		}

		info
	}

// 最大叔块数量
	fn maximum_uncle_count(&self, block: BlockNumber) -> usize {
		if block >= self.maximum_uncle_count_transition {
			self.maximum_uncle_count
		} else {
			// fallback to default value
			2
		}
	}

	// 获取父块的一些信息
	TODO:parent_step来自哪条链
	fn populate_from_parent(&self, header: &mut Header, parent: &Header) {
		let parent_step = header_step(parent, self.empty_steps_transition).expect("Header has been verified; qed");
		let current_step = self.step.inner.load();

		let current_empty_steps_len = if header.number() >= self.empty_steps_transition {
			self.empty_steps(parent_step.into(), current_step.into(), parent.hash()).len()
		} else {
			0
		};

		// 计算链的得分
		let score = calculate_score(parent_step.into(), current_step.into(), current_empty_steps_len.into());
		// 设置难度
		header.set_difficulty(score);
	}

	fn seals_internally(&self) -> Option<bool> {
		// TODO: accept a `&Call` here so we can query the validator set.
		Some(self.signer.read().is_some())
	}

// 处理消息
	fn handle_message(&self, rlp: &[u8]) -> Result<(), EngineError> {
		fn fmt_err<T: ::std::fmt::Debug>(x: T) -> EngineError {
			EngineError::MalformedMessage(format!("{:?}", x))
		}

		let rlp = Rlp::new(rlp);
		let empty_step: EmptyStep = rlp.as_val().map_err(fmt_err)?;;

		// empty_step需要传入验证人进行验证
		if empty_step.verify(&*self.validators).unwrap_or(false) {
			if self.step.inner.check_future(empty_step.step).is_ok() {
				trace!(target: "engine", "handle_message: received empty step message {:?}", empty_step);
				self.handle_empty_step_message(empty_step);
			} else {
				trace!(target: "engine", "handle_message: empty step message from the future {:?}", empty_step);
			}
		} else {
			trace!(target: "engine", "handle_message: received invalid step message {:?}", empty_step);
		};

		Ok(())
	}

	/// TODO:Attempt to seal the block internally.
	///
	/// This operation is synchronous and may (quite reasonably) not be available, in which case
	/// `Seal::None` will be returned.
	// 操作是同步的，如果不可用会返回Seal::None
	// 正常的打包
	// 这里是整个打包的过程
	// 包含验证检查，过滤消息，计算链的得分，计算难度，校验当前step是否合法，检查是否允许发起emptystep等等
	// 如果没法打包的话返回Seal::None
	fn generate_seal(&self, block: &ExecutedBlock, parent: &Header) -> Seal {
		// first check to avoid generating signature most of the time
		// (but there's still a race to the `compare_and_swap`)

		// 这里首先要检查，以避免花费太多时间签名
		// (但是还是会有比较和交换的竞争存在)
		// 如果step.can_propose是false，直接返回Seal::None
		if !self.step.can_propose.load(AtomicOrdering::SeqCst) {
			trace!(target: "engine", "Aborting seal generation. Can't propose.");
			return Seal::None;
		}

		let header = block.header();
		let parent_step: U256 = header_step(parent, self.empty_steps_transition)
			.expect("Header has been verified; qed").into();

		let step = self.step.inner.load();

		// filter messages from old and future steps and different parents
		// 过滤消息
		let empty_steps = if header.number() >= self.empty_steps_transition {
			self.empty_steps(parent_step.into(), step.into(), *header.parent_hash())
		} else {
			Vec::new()
		};

		// 通过得分计算难度
		let expected_diff = calculate_score(parent_step, step.into(), empty_steps.len().into());

		if header.difficulty() != &expected_diff {
			debug!(target: "engine", "Aborting seal generation. The step or empty_steps have changed in the meantime. {:?} != {:?}",
				   header.difficulty(), expected_diff);
			return Seal::None;
		}

		if parent_step > step.into() {
			warn!(target: "engine", "Aborting seal generation for invalid step: {} > {}", parent_step, step);
			return Seal::None;
		}

		let (validators, set_number) = match self.epoch_set(header) {
			Err(err) => {
				warn!(target: "engine", "Unable to generate seal: {}", err);
				return Seal::None;
			},
			Ok(ok) => ok,
		};

		// 这里应该是一个step被它的author发起提议
		if is_step_proposer(&*validators, header.parent_hash(), step, header.author()) {
			// this is guarded against by `can_propose` unless the block was signed
			// on the same step (implies same key) and on a different node.
			// 这是由can_propose保护的，除非该块被签名
			// 不能发生在parent step
			if parent_step == step.into() {
				warn!("Attempted to seal block on the same step as parent. Is this authority sealing with more than one node?");
				return Seal::None;
			}

			// if there are no transactions to include in the block, we don't seal and instead broadcast a signed
			// `EmptyStep(step, parent_hash)` message. If we exceed the maximum amount of `empty_step` rounds we proceed
			// with the seal.

			// 这里是不包含任何交易的区块，我们会广播一个签名后的EmptyStep消息而不是打包这个区块，如果超出了empty_step的最大值，我们继续seal
			if header.number() >= self.empty_steps_transition &&
				block.transactions().is_empty() &&
				empty_steps.len() < self.maximum_empty_steps {

				self.generate_empty_step(header.parent_hash());
				return Seal::None;
			}

			let empty_steps_rlp = if header.number() >= self.empty_steps_transition {
				let empty_steps: Vec<_> = empty_steps.iter().map(|e| e.sealed()).collect();
				Some(::rlp::encode_list(&empty_steps).into_vec())
			} else {
				None
			};

			if let Ok(signature) = self.sign(header_seal_hash(header, empty_steps_rlp.as_ref().map(|e| &**e))) {
				trace!(target: "engine", "generate_seal: Issuing a block for step {}.", step);

				// only issue the seal if we were the first to reach the compare_and_swap.
				// 各个节点之间会有交换然后比较的过程(从提议者那里拿到块，然后节点之间比较块是否是同一个)，如果是第一个得到这个块的，只确认即可
				if self.step.can_propose.compare_and_swap(true, false, AtomicOrdering::SeqCst) {
					// we can drop all accumulated empty step messages that are
					// older than the parent step since we're including them in
					// the seal
					// 在我们将消息打包之后，就可以删除所有比parent_step还早的那些消息了
					self.clear_empty_steps(parent_step);

					// report any skipped primaries between the parent block and
					// the block we're sealing, unless we have empty steps enabled

					// 举报那些跳过出块的节点，除非我们设置了emptystep开启
					if header.number() < self.empty_steps_transition {
						self.report_skipped(header, step, u64::from(parent_step) as usize, &*validators, set_number);
					}

					let mut fields = vec![
						encode(&step).into_vec(),
						encode(&(&H520::from(signature) as &[u8])).into_vec(),
					];

					if let Some(empty_steps_rlp) = empty_steps_rlp {
						fields.push(empty_steps_rlp);
					}

					return Seal::Regular(fields);
				}
			} else {
				warn!(target: "engine", "generate_seal: FAIL: Accounts secret key unavailable.");
			}
		} else {
			trace!(target: "engine", "generate_seal: {} not a proposer for step {}.",
				header.author(), step);
		}

		Seal::None
	}

	fn verify_local_seal(&self, _header: &Header) -> Result<(), Error> {
		Ok(())
	}

// 对新产生的块做检查？
	fn on_new_block(
		&self,
		block: &mut ExecutedBlock,
		epoch_begin: bool,
		_ancestry: &mut Iterator<Item=ExtendedHeader>,
	) -> Result<(), Error> {
		// with immediate transitions, we don't use the epoch mechanism anyway.
		// the genesis is always considered an epoch, but we ignore it intentionally.
		// 即时过渡，不启用epoch机制
		if self.immediate_transitions || !epoch_begin { return Ok(()) }

		// genesis is never a new block, but might as well check.
		let header = block.header().clone();
		let first = header.number() == 0;

		let mut call = |to, data| {
			let result = self.machine.execute_as_system(
				block,
				to,
				U256::max_value(), // unbounded gas? maybe make configurable.
				Some(data),
			);

			result.map_err(|e| format!("{}", e))
		};

		self.validators.on_epoch_begin(first, &header, &mut call)
	}

	/// Apply the block reward on finalisation of the block.
	// / 块打包完成了，搞定了之后分配区块奖励
	fn on_close_block(&self, block: &mut ExecutedBlock) -> Result<(), Error> {
		let mut beneficiaries = Vec::new();
		if block.header().number() >= self.empty_steps_transition {
			let empty_steps = if block.header().seal().is_empty() {
				// this is a new block, calculate rewards based on the empty steps messages we have accumulated
				// 这是一个新的区块，根据积累的消息计算奖励
				let client = match self.client.read().as_ref().and_then(|weak| weak.upgrade()) {
					Some(client) => client,
					None => {
						debug!(target: "engine", "Unable to close block: missing client ref.");
						return Err(EngineError::RequiresClient.into())
					},
				};

				let parent = client.block_header(::client::BlockId::Hash(*block.header().parent_hash()))
					.expect("hash is from parent; parent header must exist; qed")
					.decode()?;

				let parent_step = header_step(&parent, self.empty_steps_transition)?;
				let current_step = self.step.inner.load();
				self.empty_steps(parent_step.into(), current_step.into(), parent.hash())
			} else {
				// we're verifying a block, extract empty steps from the seal
				header_empty_steps(block.header())?
			};

			for empty_step in empty_steps {
				let author = empty_step.author()?;
				beneficiaries.push((author, RewardKind::EmptyStep));
			}
		}

		let author = *block.header().author();
		beneficiaries.push((author, RewardKind::Author));

		// 这里是具体的计算奖励
		let rewards: Vec<_> = match self.block_reward_contract {
			Some(ref c) if block.header().number() >= self.block_reward_contract_transition => {
				let mut call = super::default_system_or_code_call(&self.machine, block);

				let rewards = c.reward(&beneficiaries, &mut call)?;
				rewards.into_iter().map(|(author, amount)| (author, RewardKind::External, amount)).collect()
			},
			_ => {
				beneficiaries.into_iter().map(|(author, reward_kind)| (author, reward_kind, self.block_reward)).collect()
			},
		};

		block_reward::apply_block_rewards(&rewards, block, &self.machine)
	}

	/// Check the number of seal fields.
	// / 检查打包的字段是不是够，需要三个字段 达成共识的step、相应的提议者签名、一个空列表
	fn verify_block_basic(&self, header: &Header) -> Result<(), Error> {
		if header.number() >= self.validate_score_transition && *header.difficulty() >= U256::from(U128::max_value()) {
			return Err(From::from(BlockError::DifficultyOutOfBounds(
				OutOfBounds { min: None, max: Some(U256::from(U128::max_value())), found: *header.difficulty() }
			)));
		}

		// 验证时间戳
		match verify_timestamp(&self.step.inner, header_step(header, self.empty_steps_transition)?) {
			Err(BlockError::InvalidSeal) => {
				// This check runs in Phase 1 where there is no guarantee that the parent block is
				// already imported, therefore the call to `epoch_set` may fail. In that case we
				// won't report the misbehavior but this is not a concern because:
				// - Only authorities can report and it's expected that they'll be up-to-date and
				//   importing, therefore the parent header will most likely be available
				// - Even if you are an authority that is syncing the chain, the contract will most
				//   likely ignore old reports
				// - This specific check is only relevant if you're importing (since it checks
				//   against wall clock)
				if let Ok((_, set_number)) = self.epoch_set(header) {
					self.validators.report_benign(header.author(), set_number, header.number());
				}

				Err(BlockError::InvalidSeal.into())
			}
			Err(e) => Err(e.into()),
			Ok(()) => Ok(()),
		}
	}

	// 执行步骤并且验证gas限制
	/// Do the step and gas limit validation.
	fn verify_block_family(&self, header: &Header, parent: &Header) -> Result<(), Error> {
		let step = header_step(header, self.empty_steps_transition)?;
		let parent_step = header_step(parent, self.empty_steps_transition)?;

		let (validators, set_number) = self.epoch_set(header)?;

		// Ensure header is from the step after parent.
		// 确保区块头是在他的父区块之后的
		if step == parent_step
			|| (header.number() >= self.validate_step_transition && step <= parent_step) {
			trace!(target: "engine", "Multiple blocks proposed for step {}.", parent_step);

			self.validators.report_malicious(header.author(), set_number, header.number(), Default::default());
			Err(EngineError::DoubleVote(header.author().clone()))?;
		}

		// If empty step messages are enabled we will validate the messages in the seal, missing messages are not
		// reported as there's no way to tell whether the empty step message was never sent or simply not included.

		// 如果启用了emptystep消息，我们将会验证seal中存在的消息，无法检测已丢失的消息，因为无法判定消息是否没有被发送，或者根本没有存在过。
		let empty_steps_len = if header.number() >= self.empty_steps_transition {
			let validate_empty_steps = || -> Result<usize, Error> {
				let empty_steps = header_empty_steps(header)?;
				let empty_steps_len = empty_steps.len();
				for empty_step in empty_steps {
					if empty_step.step <= parent_step || empty_step.step >= step {
						Err(EngineError::InsufficientProof(
							format!("empty step proof for invalid step: {:?}", empty_step.step)))?;
					}

					if empty_step.parent_hash != *header.parent_hash() {
						Err(EngineError::InsufficientProof(
							format!("empty step proof for invalid parent hash: {:?}", empty_step.parent_hash)))?;
					}

					if !empty_step.verify(&*validators).unwrap_or(false) {
						Err(EngineError::InsufficientProof(
							format!("invalid empty step proof: {:?}", empty_step)))?;
					}
				}
				Ok(empty_steps_len)
			};

			match validate_empty_steps() {
				Ok(len) => len,
				Err(err) => {
					self.validators.report_benign(header.author(), set_number, header.number());
					return Err(err);
				},
			}
		} else {
			self.report_skipped(header, step, parent_step, &*validators, set_number);

			0
		};

		if header.number() >= self.validate_score_transition {
			let expected_difficulty = calculate_score(parent_step.into(), step.into(), empty_steps_len.into());
			if header.difficulty() != &expected_difficulty {
				return Err(From::from(BlockError::InvalidDifficulty(Mismatch { expected: expected_difficulty, found: header.difficulty().clone() })));
			}
		}

		Ok(())
	}

	// Check the validators.
	// 检查验证人
	fn verify_block_external(&self, header: &Header) -> Result<(), Error> {
		let (validators, set_number) = self.epoch_set(header)?;

		// verify signature against fixed list, but reports should go to the
		// contract itself.
		let res = verify_external(header, &*validators, self.empty_steps_transition);
		match res {
			Err(Error(ErrorKind::Engine(EngineError::NotProposer(_)), _)) => {
				self.validators.report_benign(header.author(), set_number, header.number());
			},
			Ok(_) => {
				// we can drop all accumulated empty step messages that are older than this header's step
				// 可以丢掉所有更早的空消息
				let header_step = header_step(header, self.empty_steps_transition)?;
				self.clear_empty_steps(header_step.into());
			},
			_ => {},
		}
		res
	}

	// 创世块epoch数据
	fn genesis_epoch_data(&self, header: &Header, call: &Call) -> Result<Vec<u8>, String> {
		self.validators.genesis_epoch_data(header, call)
			.map(|set_proof| combine_proofs(0, &set_proof, &[]))
	}

	fn signals_epoch_end(&self, header: &Header, aux: AuxiliaryData)
		-> super::EpochChange<EthereumMachine>
	{
		if self.immediate_transitions { return super::EpochChange::No }

		let first = header.number() == 0;
		self.validators.signals_epoch_end(first, header, aux)
	}

// 检查epoch是否已经结束了
	fn is_epoch_end(
		&self,
		chain_head: &Header,
		chain: &super::Headers<Header>,
		transition_store: &super::PendingTransitionStore,
	) -> Option<Vec<u8>> {
		// epochs only matter if we want to support light clients.
		// 如果我们想支持light客户端，epoch才有意义，比如epoch设置成3w个块一个epoch，那么epoch互相衔接，我们在客户端只需要同步最近的一个epoch即可
		if self.immediate_transitions { return None }

		let first = chain_head.number() == 0;

		// apply immediate transitions.
		// 应用即时过渡
		if let Some(change) = self.validators.is_epoch_end(first, chain_head) {
			let change = combine_proofs(chain_head.number(), &change, &[]);
			return Some(change)
		}

		let client = match self.client.read().as_ref().and_then(|weak| weak.upgrade()) {
			Some(client) => client,
			None => {
				warn!(target: "engine", "Unable to check for epoch end: missing client ref.");
				return None;
			}
		};

		// find most recently finalized blocks, then check transition store for pending transitions.
		let mut epoch_manager = self.epoch_manager.lock();
		if !epoch_manager.zoom_to(&*client, &self.machine, &*self.validators, chain_head) {
			return None;
		}


		if epoch_manager.finality_checker.subchain_head() != Some(*chain_head.parent_hash()) {
			// build new finality checker from ancestry of chain head,
			// not including chain head itself yet.

			// 从链头创建新的确定性检查器，不包含链头本身
			// 应该也是用于epoch的，轻客户端
			trace!(target: "finality", "Building finality up to parent of {} ({})",
				chain_head.hash(), chain_head.parent_hash());

			let mut hash = chain_head.parent_hash().clone();
			let mut parent_empty_steps_signers = match header_empty_steps_signers(&chain_head, self.empty_steps_transition) {
				Ok(empty_step_signers) => empty_step_signers,
				Err(_) => {
					warn!(target: "finality", "Failed to get empty step signatures from block {}", chain_head.hash());
					return None;
				}
			};

			let epoch_transition_hash = epoch_manager.epoch_transition_hash;

			// walk the chain within current epoch backwards.
			// author == ec_recover(sig) known since the blocks are in the DB.
			// the empty steps messages in a header signal approval of the parent header.

			// 在当前的epoch之后生成链
			let ancestry_iter = itertools::repeat_call(move || {
				chain(hash).and_then(|header| {
					if header.number() == 0 { return None }

					let mut signers = vec![header.author().clone()];
					signers.extend(parent_empty_steps_signers.drain(..));

					if let Ok(empty_step_signers) = header_empty_steps_signers(&header, self.empty_steps_transition) {
						let res = (hash, signers);
						hash = header.parent_hash().clone();
						parent_empty_steps_signers = empty_step_signers;

						Some(res)

					} else {
						warn!(target: "finality", "Failed to get empty step signatures from block {}", header.hash());
						None
					}
				})
			})
				.while_some()
				.take_while(|&(h, _)| h != epoch_transition_hash);

			if let Err(_) = epoch_manager.finality_checker.build_ancestry_subchain(ancestry_iter) {
				debug!(target: "engine", "inconsistent validator set within epoch");
				return None;
			}
		}

		{
			if let Ok(finalized) = epoch_manager.finality_checker.push_hash(chain_head.hash(), vec![chain_head.author().clone()]) {
				let mut finalized = finalized.into_iter();
				while let Some(finalized_hash) = finalized.next() {
					if let Some(pending) = transition_store(finalized_hash) {
						let finality_proof = ::std::iter::once(finalized_hash)
							.chain(finalized)
							.chain(epoch_manager.finality_checker.unfinalized_hashes())
							.map(|h| if h == chain_head.hash() {
								// chain closure only stores ancestry, but the chain head is also
								// unfinalized.
								chain_head.clone()
							} else {
								chain(h).expect("these headers fetched before when constructing finality checker; qed")
							})
							.collect::<Vec<Header>>();

						// this gives us the block number for `hash`, assuming it's ancestry.
						let signal_number = chain_head.number()
							- finality_proof.len() as BlockNumber
							+ 1;
						let finality_proof = ::rlp::encode_list(&finality_proof);
						epoch_manager.note_new_epoch();

						info!(target: "engine", "Applying validator set change signalled at block {}", signal_number);

						// We turn off can_propose here because upon validator set change there can
						// be two valid proposers for a single step: one from the old set and
						// one from the new.
						//
						// This way, upon encountering an epoch change, the proposer from the
						// new set will be forced to wait until the next step to avoid sealing a
						// block that breaks the invariant that the parent's step < the block's step.

						// 在这里关闭can_propose，因为提议可能来自单个step的旧set和新set，可能会相互干扰
						// 这样，当遇到epoch切换时，来自新的提议者将被迫等待到下个epoch
						self.step.can_propose.store(false, AtomicOrdering::SeqCst);
						return Some(combine_proofs(signal_number, &pending.proof, &*finality_proof));
					}
				}
			}
		}

		None
	}

	fn epoch_verifier<'a>(&self, _header: &Header, proof: &'a [u8]) -> ConstructedVerifier<'a, EthereumMachine> {
		let (signal_number, set_proof, finality_proof) = match destructure_proofs(proof) {
			Ok(x) => x,
			Err(e) => return ConstructedVerifier::Err(e),
		};

		let first = signal_number == 0;
		match self.validators.epoch_set(first, &self.machine, signal_number, set_proof) {
			Ok((list, finalize)) => {
				let verifier = Box::new(EpochVerifier {
					step: self.step.clone(),
					subchain_validators: list,
					empty_steps_transition: self.empty_steps_transition,
				});

				match finalize {
					Some(finalize) => ConstructedVerifier::Unconfirmed(verifier, finality_proof, finalize),
					None => ConstructedVerifier::Trusted(verifier),
				}
			}
			Err(e) => ConstructedVerifier::Err(e),
		}
	}

	fn register_client(&self, client: Weak<EngineClient>) {
		*self.client.write() = Some(client.clone());
		self.validators.register_client(client);
	}

// 设置签名人
	fn set_signer(&self, ap: Arc<AccountProvider>, address: Address, password: Password) {
		self.signer.write().set(ap, address, password);
	}

// 签名
	fn sign(&self, hash: H256) -> Result<Signature, Error> {
		Ok(self.signer.read().sign(hash)?)
	}

	fn snapshot_components(&self) -> Option<Box<::snapshot::SnapshotComponents>> {
		if self.immediate_transitions {
			None
		} else {
			Some(Box::new(::snapshot::PoaSnapshot))
		}
	}

	fn fork_choice(&self, new: &ExtendedHeader, current: &ExtendedHeader) -> super::ForkChoice {
		super::total_difficulty_fork_choice(new, current)
	}
}


// 测试模块
#[cfg(test)]
mod tests {
	use std::sync::Arc;
	use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
	use hash::keccak;
	use ethereum_types::{Address, H520, H256, U256};
	use header::Header;
	use rlp::encode;
	use block::*;
	use test_helpers::{
		generate_dummy_client_with_spec_and_accounts, get_temp_state_db,
		TestNotify
	};
	use account_provider::AccountProvider;
	use spec::Spec;
	use transaction::{Action, Transaction};
	use engines::{Seal, Engine, EngineError, EthEngine};
	use engines::validator_set::TestSet;
	use error::{Error, ErrorKind};
	use super::{AuthorityRoundParams, AuthorityRound, EmptyStep, SealedEmptyStep, calculate_score};

	#[test]
	fn has_valid_metadata() {
		let engine = Spec::new_test_round().engine;
		assert!(!engine.name().is_empty());
	}

	#[test]
	fn can_return_schedule() {
		let engine = Spec::new_test_round().engine;
		let schedule = engine.schedule(10000000);

		assert!(schedule.stack_limit > 0);
	}

	#[test]
	fn can_do_signature_verification_fail() {
		let engine = Spec::new_test_round().engine;
		let mut header: Header = Header::default();
		header.set_seal(vec![encode(&H520::default()).into_vec()]);

		let verify_result = engine.verify_block_external(&header);
		assert!(verify_result.is_err());
	}

	#[test]
	fn generates_seal_and_does_not_double_propose() {
		let tap = Arc::new(AccountProvider::transient_provider());
		let addr1 = tap.insert_account(keccak("1").into(), &"1".into()).unwrap();
		let addr2 = tap.insert_account(keccak("2").into(), &"2".into()).unwrap();

		let spec = Spec::new_test_round();
		let engine = &*spec.engine;
		let genesis_header = spec.genesis_header();
		let db1 = spec.ensure_db_good(get_temp_state_db(), &Default::default()).unwrap();
		let db2 = spec.ensure_db_good(get_temp_state_db(), &Default::default()).unwrap();
		let last_hashes = Arc::new(vec![genesis_header.hash()]);
		let b1 = OpenBlock::new(engine, Default::default(), false, db1, &genesis_header, last_hashes.clone(), addr1, (3141562.into(), 31415620.into()), vec![], false, &mut Vec::new().into_iter()).unwrap();
		let b1 = b1.close_and_lock().unwrap();
		let b2 = OpenBlock::new(engine, Default::default(), false, db2, &genesis_header, last_hashes, addr2, (3141562.into(), 31415620.into()), vec![], false, &mut Vec::new().into_iter()).unwrap();
		let b2 = b2.close_and_lock().unwrap();

		engine.set_signer(tap.clone(), addr1, "1".into());
		if let Seal::Regular(seal) = engine.generate_seal(b1.block(), &genesis_header) {
			assert!(b1.clone().try_seal(engine, seal).is_ok());
			// Second proposal is forbidden.
			assert!(engine.generate_seal(b1.block(), &genesis_header) == Seal::None);
		}

		engine.set_signer(tap, addr2, "2".into());
		if let Seal::Regular(seal) = engine.generate_seal(b2.block(), &genesis_header) {
			assert!(b2.clone().try_seal(engine, seal).is_ok());
			// Second proposal is forbidden.
			assert!(engine.generate_seal(b2.block(), &genesis_header) == Seal::None);
		}
	}

	#[test]
	fn checks_difficulty_in_generate_seal() {
		let tap = Arc::new(AccountProvider::transient_provider());
		let addr1 = tap.insert_account(keccak("1").into(), &"1".into()).unwrap();
		let addr2 = tap.insert_account(keccak("0").into(), &"0".into()).unwrap();

		let spec = Spec::new_test_round();
		let engine = &*spec.engine;

		let genesis_header = spec.genesis_header();
		let db1 = spec.ensure_db_good(get_temp_state_db(), &Default::default()).unwrap();
		let db2 = spec.ensure_db_good(get_temp_state_db(), &Default::default()).unwrap();
		let last_hashes = Arc::new(vec![genesis_header.hash()]);

		let b1 = OpenBlock::new(engine, Default::default(), false, db1, &genesis_header, last_hashes.clone(), addr1, (3141562.into(), 31415620.into()), vec![], false, &mut Vec::new().into_iter()).unwrap();
		let b1 = b1.close_and_lock().unwrap();
		let b2 = OpenBlock::new(engine, Default::default(), false, db2, &genesis_header, last_hashes, addr2, (3141562.into(), 31415620.into()), vec![], false, &mut Vec::new().into_iter()).unwrap();
		let b2 = b2.close_and_lock().unwrap();

		engine.set_signer(tap.clone(), addr1, "1".into());
		match engine.generate_seal(b1.block(), &genesis_header) {
			Seal::None | Seal::Proposal(_) => panic!("wrong seal"),
			Seal::Regular(_) => {
				engine.step();

				engine.set_signer(tap.clone(), addr2, "0".into());
				match engine.generate_seal(b2.block(), &genesis_header) {
					Seal::Regular(_) | Seal::Proposal(_) => panic!("sealed despite wrong difficulty"),
					Seal::None => {}
				}
			}
		}
	}

	#[test]
	fn proposer_switching() {
		let tap = AccountProvider::transient_provider();
		let addr = tap.insert_account(keccak("0").into(), &"0".into()).unwrap();
		let mut parent_header: Header = Header::default();
		parent_header.set_seal(vec![encode(&0usize).into_vec()]);
		parent_header.set_gas_limit("222222".parse::<U256>().unwrap());
		let mut header: Header = Header::default();
		header.set_number(1);
		header.set_gas_limit("222222".parse::<U256>().unwrap());
		header.set_author(addr);

		let engine = Spec::new_test_round().engine;

		// Two validators.
		// Spec starts with step 2.
		header.set_difficulty(calculate_score(U256::from(0), U256::from(2), U256::zero()));
		let signature = tap.sign(addr, Some("0".into()), header.bare_hash()).unwrap();
		header.set_seal(vec![encode(&2usize).into_vec(), encode(&(&*signature as &[u8])).into_vec()]);
		assert!(engine.verify_block_family(&header, &parent_header).is_ok());
		assert!(engine.verify_block_external(&header).is_err());
		header.set_difficulty(calculate_score(U256::from(0), U256::from(1), U256::zero()));
		let signature = tap.sign(addr, Some("0".into()), header.bare_hash()).unwrap();
		header.set_seal(vec![encode(&1usize).into_vec(), encode(&(&*signature as &[u8])).into_vec()]);
		assert!(engine.verify_block_family(&header, &parent_header).is_ok());
		assert!(engine.verify_block_external(&header).is_ok());
	}

	#[test]
	fn rejects_future_block() {
		let tap = AccountProvider::transient_provider();
		let addr = tap.insert_account(keccak("0").into(), &"0".into()).unwrap();

		let mut parent_header: Header = Header::default();
		parent_header.set_seal(vec![encode(&0usize).into_vec()]);
		parent_header.set_gas_limit("222222".parse::<U256>().unwrap());
		let mut header: Header = Header::default();
		header.set_number(1);
		header.set_gas_limit("222222".parse::<U256>().unwrap());
		header.set_author(addr);

		let engine = Spec::new_test_round().engine;

		// Two validators.
		// Spec starts with step 2.
		header.set_difficulty(calculate_score(U256::from(0), U256::from(1), U256::zero()));
		let signature = tap.sign(addr, Some("0".into()), header.bare_hash()).unwrap();
		header.set_seal(vec![encode(&1usize).into_vec(), encode(&(&*signature as &[u8])).into_vec()]);
		assert!(engine.verify_block_family(&header, &parent_header).is_ok());
		assert!(engine.verify_block_external(&header).is_ok());
		header.set_seal(vec![encode(&5usize).into_vec(), encode(&(&*signature as &[u8])).into_vec()]);
		assert!(engine.verify_block_basic(&header).is_err());
	}

	#[test]
	fn rejects_step_backwards() {
		let tap = AccountProvider::transient_provider();
		let addr = tap.insert_account(keccak("0").into(), &"0".into()).unwrap();

		let mut parent_header: Header = Header::default();
		parent_header.set_seal(vec![encode(&4usize).into_vec()]);
		parent_header.set_gas_limit("222222".parse::<U256>().unwrap());
		let mut header: Header = Header::default();
		header.set_number(1);
		header.set_gas_limit("222222".parse::<U256>().unwrap());
		header.set_author(addr);

		let engine = Spec::new_test_round().engine;

		let signature = tap.sign(addr, Some("0".into()), header.bare_hash()).unwrap();
		// Two validators.
		// Spec starts with step 2.
		header.set_seal(vec![encode(&5usize).into_vec(), encode(&(&*signature as &[u8])).into_vec()]);
		header.set_difficulty(calculate_score(U256::from(4), U256::from(5), U256::zero()));
		assert!(engine.verify_block_family(&header, &parent_header).is_ok());
		header.set_seal(vec![encode(&3usize).into_vec(), encode(&(&*signature as &[u8])).into_vec()]);
		header.set_difficulty(calculate_score(U256::from(4), U256::from(3), U256::zero()));
		assert!(engine.verify_block_family(&header, &parent_header).is_err());
	}

	#[test]
	fn reports_skipped() {
		let last_benign = Arc::new(AtomicUsize::new(0));
		let params = AuthorityRoundParams {
			step_duration: 1,
			start_step: Some(1),
			validators: Box::new(TestSet::new(Default::default(), last_benign.clone())),
			validate_score_transition: 0,
			validate_step_transition: 0,
			immediate_transitions: true,
			maximum_uncle_count_transition: 0,
			maximum_uncle_count: 0,
			empty_steps_transition: u64::max_value(),
			maximum_empty_steps: 0,
			block_reward: Default::default(),
			block_reward_contract_transition: 0,
			block_reward_contract: Default::default(),
		};

		let aura = {
			let mut c_params = ::spec::CommonParams::default();
			c_params.gas_limit_bound_divisor = 5.into();
			let machine = ::machine::EthereumMachine::regular(c_params, Default::default());
			AuthorityRound::new(params, machine).unwrap()
		};

		let mut parent_header: Header = Header::default();
		parent_header.set_seal(vec![encode(&1usize).into_vec()]);
		parent_header.set_gas_limit("222222".parse::<U256>().unwrap());
		let mut header: Header = Header::default();
		header.set_difficulty(calculate_score(U256::from(1), U256::from(3), U256::zero()));
		header.set_gas_limit("222222".parse::<U256>().unwrap());
		header.set_seal(vec![encode(&3usize).into_vec()]);

		// Do not report when signer not present.
		assert!(aura.verify_block_family(&header, &parent_header).is_ok());
		assert_eq!(last_benign.load(AtomicOrdering::SeqCst), 0);

		aura.set_signer(Arc::new(AccountProvider::transient_provider()), Default::default(), "".into());

		// Do not report on steps skipped between genesis and first block.
		header.set_number(1);
		assert!(aura.verify_block_family(&header, &parent_header).is_ok());
		assert_eq!(last_benign.load(AtomicOrdering::SeqCst), 0);

		// Report on skipped steps otherwise.
		header.set_number(2);
		assert!(aura.verify_block_family(&header, &parent_header).is_ok());
		assert_eq!(last_benign.load(AtomicOrdering::SeqCst), 2);
	}

	#[test]
	fn test_uncles_transition() {
		let last_benign = Arc::new(AtomicUsize::new(0));
		let params = AuthorityRoundParams {
			step_duration: 1,
			start_step: Some(1),
			validators: Box::new(TestSet::new(Default::default(), last_benign.clone())),
			validate_score_transition: 0,
			validate_step_transition: 0,
			immediate_transitions: true,
			maximum_uncle_count_transition: 1,
			maximum_uncle_count: 0,
			empty_steps_transition: u64::max_value(),
			maximum_empty_steps: 0,
			block_reward: Default::default(),
			block_reward_contract_transition: 0,
			block_reward_contract: Default::default(),
		};

		let aura = {
			let mut c_params = ::spec::CommonParams::default();
			c_params.gas_limit_bound_divisor = 5.into();
			let machine = ::machine::EthereumMachine::regular(c_params, Default::default());
			AuthorityRound::new(params, machine).unwrap()
		};

		assert_eq!(aura.maximum_uncle_count(0), 2);
		assert_eq!(aura.maximum_uncle_count(1), 0);
		assert_eq!(aura.maximum_uncle_count(100), 0);
	}

    #[test]
    #[should_panic(expected="counter is too high")]
    fn test_counter_increment_too_high() {
        use super::Step;
        let step = Step {
            calibrate: false,
            inner: AtomicUsize::new(::std::usize::MAX),
            duration: 1,
        };
        step.increment();
	}

	#[test]
	#[should_panic(expected="counter is too high")]
	fn test_counter_duration_remaining_too_high() {
		use super::Step;
		let step = Step {
			calibrate: false,
			inner: AtomicUsize::new(::std::usize::MAX),
			duration: 1,
		};
		step.duration_remaining();
	}

	#[test]
	#[should_panic(expected="authority_round: step duration can't be zero")]
	fn test_step_duration_zero() {
		let last_benign = Arc::new(AtomicUsize::new(0));
		let params = AuthorityRoundParams {
			step_duration: 0,
			start_step: Some(1),
			validators: Box::new(TestSet::new(Default::default(), last_benign.clone())),
			validate_score_transition: 0,
			validate_step_transition: 0,
			immediate_transitions: true,
			maximum_uncle_count_transition: 0,
			maximum_uncle_count: 0,
			empty_steps_transition: u64::max_value(),
			maximum_empty_steps: 0,
			block_reward: Default::default(),
			block_reward_contract_transition: 0,
			block_reward_contract: Default::default(),
		};

		let mut c_params = ::spec::CommonParams::default();
		c_params.gas_limit_bound_divisor = 5.into();
		let machine = ::machine::EthereumMachine::regular(c_params, Default::default());
		AuthorityRound::new(params, machine).unwrap();
	}

	fn setup_empty_steps() -> (Spec, Arc<AccountProvider>, Vec<Address>) {
		let spec = Spec::new_test_round_empty_steps();
		let tap = Arc::new(AccountProvider::transient_provider());

		let addr1 = tap.insert_account(keccak("1").into(), &"1".into()).unwrap();
		let addr2 = tap.insert_account(keccak("0").into(), &"0".into()).unwrap();

		let accounts = vec![addr1, addr2];

		(spec, tap, accounts)
	}

	fn empty_step(engine: &EthEngine, step: usize, parent_hash: &H256) -> EmptyStep {
		let empty_step_rlp = super::empty_step_rlp(step, parent_hash);
		let signature = engine.sign(keccak(&empty_step_rlp)).unwrap().into();
		let parent_hash = parent_hash.clone();
		EmptyStep { step, signature, parent_hash }
	}

	fn sealed_empty_step(engine: &EthEngine, step: usize, parent_hash: &H256) -> SealedEmptyStep {
		let empty_step_rlp = super::empty_step_rlp(step, parent_hash);
		let signature = engine.sign(keccak(&empty_step_rlp)).unwrap().into();
		SealedEmptyStep { signature, step }
	}

	#[test]
	fn broadcast_empty_step_message() {
		let (spec, tap, accounts) = setup_empty_steps();

		let addr1 = accounts[0];

		let engine = &*spec.engine;
		let genesis_header = spec.genesis_header();
		let db1 = spec.ensure_db_good(get_temp_state_db(), &Default::default()).unwrap();

		let last_hashes = Arc::new(vec![genesis_header.hash()]);

		let client = generate_dummy_client_with_spec_and_accounts(Spec::new_test_round_empty_steps, None);
		let notify = Arc::new(TestNotify::default());
		client.add_notify(notify.clone());
		engine.register_client(Arc::downgrade(&client) as _);

		engine.set_signer(tap.clone(), addr1, "1".into());

		let b1 = OpenBlock::new(engine, Default::default(), false, db1, &genesis_header, last_hashes.clone(), addr1, (3141562.into(), 31415620.into()), vec![], false, &mut Vec::new().into_iter()).unwrap();
		let b1 = b1.close_and_lock().unwrap();

		// the block is empty so we don't seal and instead broadcast an empty step message
		assert_eq!(engine.generate_seal(b1.block(), &genesis_header), Seal::None);

		// spec starts with step 2
		let empty_step_rlp = encode(&empty_step(engine, 2, &genesis_header.hash())).into_vec();

		// we've received the message
		assert!(notify.messages.read().contains(&empty_step_rlp));
	}

	#[test]
	fn seal_with_empty_steps() {
		let (spec, tap, accounts) = setup_empty_steps();

		let addr1 = accounts[0];
		let addr2 = accounts[1];

		let engine = &*spec.engine;
		let genesis_header = spec.genesis_header();
		let db1 = spec.ensure_db_good(get_temp_state_db(), &Default::default()).unwrap();
		let db2 = spec.ensure_db_good(get_temp_state_db(), &Default::default()).unwrap();

		let last_hashes = Arc::new(vec![genesis_header.hash()]);

		let client = generate_dummy_client_with_spec_and_accounts(Spec::new_test_round_empty_steps, None);
		let notify = Arc::new(TestNotify::default());
		client.add_notify(notify.clone());
		engine.register_client(Arc::downgrade(&client) as _);

		// step 2
		let b1 = OpenBlock::new(engine, Default::default(), false, db1, &genesis_header, last_hashes.clone(), addr1, (3141562.into(), 31415620.into()), vec![], false, &mut Vec::new().into_iter()).unwrap();
		let b1 = b1.close_and_lock().unwrap();

		// since the block is empty it isn't sealed and we generate empty steps
		engine.set_signer(tap.clone(), addr1, "1".into());
		assert_eq!(engine.generate_seal(b1.block(), &genesis_header), Seal::None);
		engine.step();

		// step 3
		let mut b2 = OpenBlock::new(engine, Default::default(), false, db2, &genesis_header, last_hashes.clone(), addr2, (3141562.into(), 31415620.into()), vec![], false, &mut Vec::new().into_iter()).unwrap();
		b2.push_transaction(Transaction {
			action: Action::Create,
			nonce: U256::from(0),
			gas_price: U256::from(3000),
			gas: U256::from(53_000),
			value: U256::from(1),
			data: vec![],
		}.fake_sign(addr2), None).unwrap();
		let b2 = b2.close_and_lock().unwrap();

		// we will now seal a block with 1tx and include the accumulated empty step message
		engine.set_signer(tap.clone(), addr2, "0".into());
		if let Seal::Regular(seal) = engine.generate_seal(b2.block(), &genesis_header) {
			engine.set_signer(tap.clone(), addr1, "1".into());
			let empty_step2 = sealed_empty_step(engine, 2, &genesis_header.hash());
			let empty_steps = ::rlp::encode_list(&vec![empty_step2]);

			assert_eq!(seal[0], encode(&3usize).into_vec());
			assert_eq!(seal[2], empty_steps.into_vec());
		}
	}

	#[test]
	fn seal_empty_block_with_empty_steps() {
		let (spec, tap, accounts) = setup_empty_steps();

		let addr1 = accounts[0];
		let addr2 = accounts[1];

		let engine = &*spec.engine;
		let genesis_header = spec.genesis_header();
		let db1 = spec.ensure_db_good(get_temp_state_db(), &Default::default()).unwrap();
		let db2 = spec.ensure_db_good(get_temp_state_db(), &Default::default()).unwrap();
		let db3 = spec.ensure_db_good(get_temp_state_db(), &Default::default()).unwrap();

		let last_hashes = Arc::new(vec![genesis_header.hash()]);

		let client = generate_dummy_client_with_spec_and_accounts(Spec::new_test_round_empty_steps, None);
		let notify = Arc::new(TestNotify::default());
		client.add_notify(notify.clone());
		engine.register_client(Arc::downgrade(&client) as _);

		// step 2
		let b1 = OpenBlock::new(engine, Default::default(), false, db1, &genesis_header, last_hashes.clone(), addr1, (3141562.into(), 31415620.into()), vec![], false, &mut Vec::new().into_iter()).unwrap();
		let b1 = b1.close_and_lock().unwrap();

		// since the block is empty it isn't sealed and we generate empty steps
		engine.set_signer(tap.clone(), addr1, "1".into());
		assert_eq!(engine.generate_seal(b1.block(), &genesis_header), Seal::None);
		engine.step();

		// step 3
		let b2 = OpenBlock::new(engine, Default::default(), false, db2, &genesis_header, last_hashes.clone(), addr2, (3141562.into(), 31415620.into()), vec![], false, &mut Vec::new().into_iter()).unwrap();
		let b2 = b2.close_and_lock().unwrap();
		engine.set_signer(tap.clone(), addr2, "0".into());
		assert_eq!(engine.generate_seal(b2.block(), &genesis_header), Seal::None);
		engine.step();

		// step 4
		// the spec sets the maximum_empty_steps to 2 so we will now seal an empty block and include the empty step messages
		let b3 = OpenBlock::new(engine, Default::default(), false, db3, &genesis_header, last_hashes.clone(), addr1, (3141562.into(), 31415620.into()), vec![], false, &mut Vec::new().into_iter()).unwrap();
		let b3 = b3.close_and_lock().unwrap();

		engine.set_signer(tap.clone(), addr1, "1".into());
		if let Seal::Regular(seal) = engine.generate_seal(b3.block(), &genesis_header) {
			let empty_step2 = sealed_empty_step(engine, 2, &genesis_header.hash());
			engine.set_signer(tap.clone(), addr2, "0".into());
			let empty_step3 = sealed_empty_step(engine, 3, &genesis_header.hash());

			let empty_steps = ::rlp::encode_list(&vec![empty_step2, empty_step3]);

			assert_eq!(seal[0], encode(&4usize).into_vec());
			assert_eq!(seal[2], empty_steps.into_vec());
		}
	}

	#[test]
	fn reward_empty_steps() {
		let (spec, tap, accounts) = setup_empty_steps();

		let addr1 = accounts[0];

		let engine = &*spec.engine;
		let genesis_header = spec.genesis_header();
		let db1 = spec.ensure_db_good(get_temp_state_db(), &Default::default()).unwrap();
		let db2 = spec.ensure_db_good(get_temp_state_db(), &Default::default()).unwrap();

		let last_hashes = Arc::new(vec![genesis_header.hash()]);

		let client = generate_dummy_client_with_spec_and_accounts(Spec::new_test_round_empty_steps, None);
		engine.register_client(Arc::downgrade(&client) as _);

		// step 2
		let b1 = OpenBlock::new(engine, Default::default(), false, db1, &genesis_header, last_hashes.clone(), addr1, (3141562.into(), 31415620.into()), vec![], false, &mut Vec::new().into_iter()).unwrap();
		let b1 = b1.close_and_lock().unwrap();

		// since the block is empty it isn't sealed and we generate empty steps
		engine.set_signer(tap.clone(), addr1, "1".into());
		assert_eq!(engine.generate_seal(b1.block(), &genesis_header), Seal::None);
		engine.step();

		// step 3
		// the signer of the accumulated empty step message should be rewarded
		let b2 = OpenBlock::new(engine, Default::default(), false, db2, &genesis_header, last_hashes.clone(), addr1, (3141562.into(), 31415620.into()), vec![], false, &mut Vec::new().into_iter()).unwrap();
		let addr1_balance = b2.block().state().balance(&addr1).unwrap();

		// after closing the block `addr1` should be reward twice, one for the included empty step message and another for block creation
		let b2 = b2.close_and_lock().unwrap();

		// the spec sets the block reward to 10
		assert_eq!(b2.block().state().balance(&addr1).unwrap(), addr1_balance + (10 * 2))
	}

	#[test]
	fn verify_seal_empty_steps() {
		let (spec, tap, accounts) = setup_empty_steps();
		let addr1 = accounts[0];
		let addr2 = accounts[1];
		let engine = &*spec.engine;

		let mut parent_header: Header = Header::default();
		parent_header.set_seal(vec![encode(&0usize).into_vec()]);
		parent_header.set_gas_limit("222222".parse::<U256>().unwrap());

		let mut header: Header = Header::default();
		header.set_parent_hash(parent_header.hash());
		header.set_number(1);
		header.set_gas_limit("222222".parse::<U256>().unwrap());
		header.set_author(addr1);

		let signature = tap.sign(addr1, Some("1".into()), header.bare_hash()).unwrap();

		// empty step with invalid step
		let empty_steps = vec![SealedEmptyStep { signature: 0.into(), step: 2 }];
		header.set_seal(vec![
			encode(&2usize).into_vec(),
			encode(&(&*signature as &[u8])).into_vec(),
			::rlp::encode_list(&empty_steps).into_vec(),
		]);

		assert!(match engine.verify_block_family(&header, &parent_header) {
			Err(Error(ErrorKind::Engine(EngineError::InsufficientProof(ref s)), _))
				if s.contains("invalid step") => true,
			_ => false,
		});

		// empty step with invalid signature
		let empty_steps = vec![SealedEmptyStep { signature: 0.into(), step: 1 }];
		header.set_seal(vec![
			encode(&2usize).into_vec(),
			encode(&(&*signature as &[u8])).into_vec(),
			::rlp::encode_list(&empty_steps).into_vec(),
		]);

		assert!(match engine.verify_block_family(&header, &parent_header) {
			Err(Error(ErrorKind::Engine(EngineError::InsufficientProof(ref s)), _))
				if s.contains("invalid empty step proof") => true,
			_ => false,
		});

		// empty step with valid signature from incorrect proposer for step
		engine.set_signer(tap.clone(), addr1, "1".into());
		let empty_steps = vec![sealed_empty_step(engine, 1, &parent_header.hash())];
		header.set_seal(vec![
			encode(&2usize).into_vec(),
			encode(&(&*signature as &[u8])).into_vec(),
			::rlp::encode_list(&empty_steps).into_vec(),
		]);

		assert!(match engine.verify_block_family(&header, &parent_header) {
			Err(Error(ErrorKind::Engine(EngineError::InsufficientProof(ref s)), _))
				if s.contains("invalid empty step proof") => true,
			_ => false,
		});

		// valid empty steps
		engine.set_signer(tap.clone(), addr1, "1".into());
		let empty_step2 = sealed_empty_step(engine, 2, &parent_header.hash());
		engine.set_signer(tap.clone(), addr2, "0".into());
		let empty_step3 = sealed_empty_step(engine, 3, &parent_header.hash());

		let empty_steps = vec![empty_step2, empty_step3];
		header.set_difficulty(calculate_score(U256::from(0), U256::from(4), U256::from(2)));
		let signature = tap.sign(addr1, Some("1".into()), header.bare_hash()).unwrap();
		header.set_seal(vec![
			encode(&4usize).into_vec(),
			encode(&(&*signature as &[u8])).into_vec(),
			::rlp::encode_list(&empty_steps).into_vec(),
		]);

		assert!(engine.verify_block_family(&header, &parent_header).is_ok());
	}

	#[test]
	fn block_reward_contract() {
		let spec = Spec::new_test_round_block_reward_contract();
		let tap = Arc::new(AccountProvider::transient_provider());

		let addr1 = tap.insert_account(keccak("1").into(), &"1".into()).unwrap();

		let engine = &*spec.engine;
		let genesis_header = spec.genesis_header();
		let db1 = spec.ensure_db_good(get_temp_state_db(), &Default::default()).unwrap();
		let db2 = spec.ensure_db_good(get_temp_state_db(), &Default::default()).unwrap();

		let last_hashes = Arc::new(vec![genesis_header.hash()]);

		let client = generate_dummy_client_with_spec_and_accounts(
			Spec::new_test_round_block_reward_contract,
			None,
		);
		engine.register_client(Arc::downgrade(&client) as _);

		// step 2
		let b1 = OpenBlock::new(
			engine,
			Default::default(),
			false,
			db1,
			&genesis_header,
			last_hashes.clone(),
			addr1,
			(3141562.into(), 31415620.into()),
			vec![],
			false,
			&mut Vec::new().into_iter(),
		).unwrap();
		let b1 = b1.close_and_lock().unwrap();

		// since the block is empty it isn't sealed and we generate empty steps
		engine.set_signer(tap.clone(), addr1, "1".into());
		assert_eq!(engine.generate_seal(b1.block(), &genesis_header), Seal::None);
		engine.step();

		// step 3
		// the signer of the accumulated empty step message should be rewarded
		let b2 = OpenBlock::new(
			engine,
			Default::default(),
			false,
			db2,
			&genesis_header,
			last_hashes.clone(),
			addr1,
			(3141562.into(), 31415620.into()),
			vec![],
			false,
			&mut Vec::new().into_iter(),
		).unwrap();
		let addr1_balance = b2.block().state().balance(&addr1).unwrap();

		// after closing the block `addr1` should be reward twice, one for the included empty step
		// message and another for block creation
		let b2 = b2.close_and_lock().unwrap();

		// the contract rewards (1000 + kind) for each benefactor/reward kind
		assert_eq!(
			b2.block().state().balance(&addr1).unwrap(),
			addr1_balance + (1000 + 0) + (1000 + 2),
		)
	}
}