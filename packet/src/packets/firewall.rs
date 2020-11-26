use crate::{serialize::*, deserialize::* };
use alloc::{vec, vec::Vec};
use core::mem::size_of;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FirewallAction {
    Deny = 0,
    Allow = 1,
}
impl  Default for FirewallAction {
    fn default() -> Self {
        FirewallAction::Allow
    }
}
impl From<u8> for FirewallAction {
    fn from(t: u8) -> Self {
        match t {
            0 => FirewallAction::Deny,
            1 => FirewallAction::Allow,
            _ => FirewallAction::Allow,
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct FirewallRule {
    pub source_ip: u32,
    pub dest_ip: u32,
    pub source_mask: u32,
    pub dest_mask: u32,
    pub source_port: u16,
    pub dest_port: u16,
    pub protocol: u8,
    pub priority: usize,
    pub action: FirewallAction,
}

#[derive(Debug, PartialEq)]
pub struct FirewallRuleList {
    pub rules: Vec<FirewallRule>,
}

#[derive(Debug, PartialEq)]
pub enum FirewallMessage {
    /* 0 */ Error, 
    /* 1 */ QueryRules, 
    /* 2 */ SetDefault(FirewallRule),
    /* 3 */ SetRule(FirewallRule),
    /* 4 */ RuleList(Vec<FirewallRule>),
    /* 5 */ DeleteRule(usize),
}

impl EvalSize for FirewallRule {
    fn eval_size(&self) -> usize {
        size_of::<u32>() * 4
        + size_of::<u16>() * 2
        + size_of::<u8>() * 2
        + size_of::<usize>()
    }
}

impl EvalSize for FirewallMessage {
    fn eval_size(&self) -> usize {
        match self {
            FirewallMessage::Error | FirewallMessage::QueryRules => core::mem::size_of::<i32>(),
            FirewallMessage::RuleList(list) =>(
                size_of::<i32>()
                + list.as_slice().eval_size()
            ),
            FirewallMessage::SetDefault(rule) | FirewallMessage::SetRule(rule) => (
                size_of::<i32>() + rule.eval_size()
            ),
            FirewallMessage::DeleteRule(_) => size_of::<i32>() + size_of::<usize>(),
        }
    }
}

impl Serialize for FirewallRule {
    fn serialize<'s>(&self, serializer: Serializer<'s>) -> Serializer<'s> {
        serializer
            .serialize(&self.source_ip)
            .serialize(&self.source_mask)
            .serialize(&self.dest_ip)
            .serialize(&self.dest_mask)
            .serialize(&self.source_port)
            .serialize(&self.dest_port)
            .serialize(&self.protocol)
            .serialize(&self.priority)
            .serialize(&(self.action as u8))
    }
}

impl Deserialize<FirewallRule> for FirewallRule {
    fn deserialize<'a>(deserializer: &mut Deserializer<'a>) -> DeserializeResult<FirewallRule> {
        Ok(FirewallRule {
            source_ip: deserializer.deserialize_u32()?,
            source_mask: deserializer.deserialize_u32()?,
            dest_ip: deserializer.deserialize_u32()?,
            dest_mask: deserializer.deserialize()?,
            source_port: deserializer.deserialize()?,
            dest_port: deserializer.deserialize()?,
            protocol: deserializer.deserialize()?,
            priority: deserializer.deserialize()?,
            action: FirewallAction::from(deserializer.deserialize_u8()?),
        })
    }
}

impl Serialize for FirewallRuleList {
    fn serialize<'s>(&self, serializer: Serializer<'s>) -> Serializer<'s> {
        serializer.serialize(&&self.rules[..])
    }
}

impl Deserialize<FirewallRuleList> for FirewallRuleList {
    fn deserialize<'a>(deserializer: &mut Deserializer<'a>) -> DeserializeResult<FirewallRuleList> {
        Ok(FirewallRuleList {
            rules: deserializer.deserialize_vec(vec![])?
        })
    }
}

impl Serialize for FirewallMessage {
    fn serialize<'s>(&self, serializer: Serializer<'s>) -> Serializer<'s> {
        match self {
            FirewallMessage::Error => serializer.serialize(&0),
            FirewallMessage::QueryRules => serializer.serialize(&1),
            FirewallMessage::SetDefault(rule) => serializer.serialize(&2).serialize(rule),
            FirewallMessage::SetRule(rule) => serializer.serialize(&3).serialize(rule),
            FirewallMessage::RuleList(rules) => serializer.serialize(&4).serialize(&&rules[..]),
            FirewallMessage::DeleteRule(id) => serializer.serialize(&5).serialize(id),
        }
    }
}

impl Deserialize<FirewallMessage> for FirewallMessage {
    fn deserialize<'a>(deserializer: &mut Deserializer<'a>) -> DeserializeResult<FirewallMessage> {
        let msg_type = deserializer.deserialize_i32()?;
        let msg = match msg_type {
            1 => FirewallMessage::QueryRules,
            2 => FirewallMessage::SetDefault(deserializer.deserialize()?),
            3 => FirewallMessage::SetRule(deserializer.deserialize()?),
            4 => FirewallMessage::RuleList(deserializer.deserialize_vec(vec![])?),
            5 => FirewallMessage::DeleteRule(deserializer.deserialize()?),
            _ => FirewallMessage::Error,
        };
        Ok(msg)
    }
}