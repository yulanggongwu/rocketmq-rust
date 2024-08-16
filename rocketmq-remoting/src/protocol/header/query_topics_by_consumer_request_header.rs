/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use serde::Deserialize;
use serde::Serialize;

use crate::protocol::command_custom_header::CommandCustomHeader;
use crate::protocol::command_custom_header::FromMap;
use crate::rpc::rpc_request_header::RpcRequestHeader;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryTopicsByConsumerRequestHeader {
    #[serde(rename = "group")]
    pub group: String,

    #[serde(flatten)]
    pub rpc_request_header: Option<RpcRequestHeader>,
}

impl QueryTopicsByConsumerRequestHeader {
    pub const GROUP: &'static str = "group";

    pub fn get_group(&self) -> &String {
        &self.group
    }
    pub fn set_group(&mut self, group: String) {
        self.group = group;
    }
}

impl CommandCustomHeader for QueryTopicsByConsumerRequestHeader {
    fn to_map(&self) -> Option<std::collections::HashMap<String, String>> {
        let mut map = std::collections::HashMap::new();
        map.insert(Self::GROUP.to_string(), self.group.clone());
        if let Some(value) = self.rpc_request_header.as_ref() {
            if let Some(value) = value.to_map() {
                map.extend(value);
            }
        }
        Some(map)
    }
}

impl FromMap for QueryTopicsByConsumerRequestHeader {
    type Target = Self;

    fn from(map: &std::collections::HashMap<String, String>) -> Option<Self::Target> {
        Some(QueryTopicsByConsumerRequestHeader {
            group: map.get(Self::GROUP).cloned().unwrap_or_default(),
            rpc_request_header: <RpcRequestHeader as FromMap>::from(map),
        })
    }
}