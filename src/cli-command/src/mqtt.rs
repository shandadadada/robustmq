// Copyright 2023 RobustMQ Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub struct MqttCliCommandParam {
    pub server: String,
    pub action: String,
}

pub enum MqttActionType {
    STATUS,
}

impl From<String> for MqttActionType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "status" => MqttActionType::STATUS,
            _ => panic!("Invalid action type {}", s),
        }
    }
}

pub struct MqttBrokerCommand {}

impl MqttBrokerCommand {
    pub fn new() -> Self {
        return MqttBrokerCommand {};
    }

    pub async fn start(&self, params: MqttCliCommandParam) {
        let action_type = MqttActionType::from(params.action.clone());
        match action_type {
            MqttActionType::STATUS => {
                println!("mqtt status");
            }
        }
    }
}