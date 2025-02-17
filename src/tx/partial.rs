use std::io;

use super::TransactionOutput;
use crate::{
    crypto::{
        keypair::PublicKey,
        types::{DrkTokenId, DrkValueBlind},
        BurnRevealedValues, Proof,
    },
    impl_vec,
    util::serial::{Decodable, Encodable, SerialDecodable, SerialEncodable, VarInt},
    Result,
};

#[derive(SerialEncodable, SerialDecodable)]
pub struct PartialTransaction {
    pub clear_inputs: Vec<PartialTransactionClearInput>,
    pub inputs: Vec<PartialTransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

#[derive(SerialEncodable, SerialDecodable)]
pub struct PartialTransactionClearInput {
    pub value: u64,
    pub token_id: DrkTokenId,
    pub value_blind: DrkValueBlind,
    pub token_blind: DrkValueBlind,
    pub signature_public: PublicKey,
}

#[derive(SerialEncodable, SerialDecodable)]
pub struct PartialTransactionInput {
    pub burn_proof: Proof,
    pub revealed: BurnRevealedValues,
}

impl_vec!(PartialTransactionClearInput);
impl_vec!(PartialTransactionInput);
