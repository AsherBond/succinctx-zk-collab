use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{Witness, WitnessWrite};

use crate::builder::CircuitBuilder;
use crate::eth::vars::BLSPubkeyVariable;
use crate::ethutils::beacon::BeaconValidator;
use crate::utils::{bytes, bytes32, hex};
use crate::vars::{BoolVariable, Bytes32Variable, CircuitVariable, U256Variable};

/*
proof frequency: one per hour

function computeValidatorBalanceSum(validatorPubkeys, timestamp) => uint256: {
    let slot = timestamp_to_slot(slot);
    validators = get_validators(slot)
    sum = 0
    for validator in validators:
        for i in range(len(validatorPubkeys)):
            if validator.pubkey == validatorPubkeys[i]:
                sum += validator.effective_balance
    return sum
}

function getValidatorSlashStatus(validatorPubkeys, timestamp) => bool[] {
    let slot = timestamp_to_slot(slot);
    validators = get_validators(slot)
    slashing_statuses = []
    for validator in validators:
        for i in range(len(validatorPubkeys)):
            if validator.pubkey == validatorPubkeys[i]:
                slashing_statuses.append(validator.slashed)
    return slashing_statuses
}
*/

#[derive(Debug, Clone, Copy)]
pub struct BeaconValidatorVariable {
    pub pubkey: BLSPubkeyVariable,
    pub withdrawal_credentials: Bytes32Variable,
    pub effective_balance: U256Variable,
    pub slashed: BoolVariable,
    pub activation_eligibility_epoch: U256Variable,
    pub activation_epoch: U256Variable,
    pub exit_epoch: U256Variable,
    pub withdrawable_epoch: U256Variable,
}

impl CircuitVariable for BeaconValidatorVariable {
    type ValueType = BeaconValidator;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self {
            pubkey: BLSPubkeyVariable::init(builder),
            withdrawal_credentials: Bytes32Variable::init(builder),
            effective_balance: U256Variable::init(builder),
            slashed: BoolVariable::init(builder),
            activation_eligibility_epoch: U256Variable::init(builder),
            activation_epoch: U256Variable::init(builder),
            exit_epoch: U256Variable::init(builder),
            withdrawable_epoch: U256Variable::init(builder),
        }
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType,
    ) -> Self {
        Self {
            pubkey: BLSPubkeyVariable::constant(builder, bytes!(value.pubkey)),
            withdrawal_credentials: Bytes32Variable::constant(
                builder,
                bytes32!(value.withdrawal_credentials),
            ),
            effective_balance: U256Variable::constant(builder, value.effective_balance.into()),
            slashed: BoolVariable::constant(builder, value.slashed),
            activation_eligibility_epoch: U256Variable::constant(
                builder,
                value.activation_eligibility_epoch.into(),
            ),
            activation_epoch: U256Variable::constant(builder, value.activation_epoch.into()),
            exit_epoch: U256Variable::constant(builder, value.exit_epoch.unwrap_or(0).into()),
            withdrawable_epoch: U256Variable::constant(
                builder,
                value.withdrawable_epoch.unwrap_or(0).into(),
            ),
        }
    }

    fn targets(&self) -> Vec<Target> {
        let mut targets = Vec::new();
        targets.extend(self.pubkey.targets());
        targets.extend(self.withdrawal_credentials.targets());
        targets.extend(self.effective_balance.targets());
        targets.extend(self.slashed.targets());
        targets.extend(self.activation_eligibility_epoch.targets());
        targets.extend(self.activation_epoch.targets());
        targets.extend(self.exit_epoch.targets());
        targets.extend(self.withdrawable_epoch.targets());
        targets
    }

    fn from_targets(targets: &[Target]) -> Self {
        let mut ptr = 0;
        let pubkey = BLSPubkeyVariable::from_targets(&targets[ptr..ptr + 384]);
        ptr += 384;
        let withdrawal_credentials = Bytes32Variable::from_targets(&targets[ptr..ptr + 256]);
        ptr += 256;
        let effective_balance = U256Variable::from_targets(&targets[ptr..ptr + 4]);
        ptr += 4;
        let slashed = BoolVariable::from_targets(&targets[ptr..ptr + 1]);
        ptr += 1;
        let activation_eligibility_epoch = U256Variable::from_targets(&targets[ptr..ptr + 4]);
        ptr += 4;
        let activation_epoch = U256Variable::from_targets(&targets[ptr..ptr + 4]);
        ptr += 4;
        let exit_epoch = U256Variable::from_targets(&targets[ptr..ptr + 4]);
        ptr += 4;
        let withdrawable_epoch = U256Variable::from_targets(&targets[ptr..ptr + 4]);
        Self {
            pubkey,
            withdrawal_credentials,
            effective_balance,
            slashed,
            activation_eligibility_epoch,
            activation_epoch,
            exit_epoch,
            withdrawable_epoch,
        }
    }

    fn value<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType {
        BeaconValidator {
            pubkey: hex!(self.pubkey.value(witness)),
            withdrawal_credentials: hex!(self.withdrawal_credentials.value(witness)),
            effective_balance: self.effective_balance.value(witness).as_u64(),
            slashed: self.slashed.value(witness),
            activation_eligibility_epoch: self.activation_eligibility_epoch.value(witness).as_u64(),
            activation_epoch: self.activation_epoch.value(witness).as_u64(),
            exit_epoch: Some(self.exit_epoch.value(witness).as_u64()),
            withdrawable_epoch: Some(self.withdrawable_epoch.value(witness).as_u64()),
        }
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType) {
        self.pubkey.set(witness, bytes!(value.pubkey));
        self.withdrawal_credentials
            .set(witness, bytes32!(value.withdrawal_credentials));
        self.effective_balance
            .set(witness, value.effective_balance.into());
        self.slashed.set(witness, value.slashed);
        self.activation_eligibility_epoch
            .set(witness, value.activation_eligibility_epoch.into());
        self.activation_epoch
            .set(witness, value.activation_epoch.into());
        self.exit_epoch
            .set(witness, value.exit_epoch.unwrap_or(0).into());
        self.withdrawable_epoch
            .set(witness, value.withdrawable_epoch.unwrap_or(0).into());
    }
}