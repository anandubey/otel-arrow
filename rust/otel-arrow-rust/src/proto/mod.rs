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

// disable some of the rust/clippy lints that we are not able to control via prost codegen
#![allow(
    clippy::must_use_candidate,
    unused_qualifications,
    missing_docs,
    unused_results
)]

#[path = "."]
pub mod opentelemetry {
    #[path = "."]
    pub mod arrow {
        #[path = "opentelemetry.proto.experimental.arrow.v1.rs"]
        pub mod v1;
    }
}
