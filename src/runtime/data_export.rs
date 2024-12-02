// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of PSH.
//
// PSH is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// PSH is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Performance Savior Home (PSH). If not,
// see <https://www.gnu.org/licenses/>.

use profiling::data_export::measurement::Point;
use profiling::data_export::metric::Sample;
use rinfluxdb::line_protocol::LineBuilder;
use wasmtime::component::Linker;

use crate::services::pb::DataRequest;
use crate::services::pb::Metadata;
use crate::services::pb::MetricMeta;
use crate::services::rpc::RpcClient;

wasmtime::component::bindgen!({
    path: "psh-sdk-wit/wit/deps/data-export",
    world: "imports",
    // https://github.com/bytecodealliance/wasmtime/pull/8310
    // wasmtime have added a config in bindgen! macro to allow user specify
    // whether they want a function be able to trap(outer wasmtime::Result).
    // by default the value is false, we use true here to compatible with our
    // previous implementations.
    trappable_imports: true,
});

#[derive(Clone)]
pub struct DataExportCtx {
    pub rpc_client: Option<RpcClient>,
}

impl profiling::data_export::file::Host for DataExportCtx {
    fn export_bytes(&mut self, bytes: Vec<u8>) -> wasmtime::Result<Result<(), String>> {
        let Some(rpc_client) = &mut self.rpc_client else {
            return Ok(Ok(()));
        };
        let metadata = Metadata {
            r#type: "file".to_string(),
            size: bytes.len() as _,
            metric_meta: None,
        };
        let req = DataRequest {
            metadata: Some(metadata),
            payload: bytes,
        };
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(rpc_client.send_data(req))?;
        Ok(Ok(()))
    }
}

impl profiling::data_export::metric::Host for DataExportCtx {
    fn export_sample(&mut self, mut sample: Sample) -> wasmtime::Result<Result<(), String>> {
        let Some(rpc_client) = &mut self.rpc_client else {
            return Ok(Ok(()));
        };

        let instance_id = rpc_client
            .instance_id()
            .unwrap_or_else(|_| "unknown".to_string());
        sample.tags.push(("instance_id".to_string(), instance_id));

        let payload = {
            let mut lb = LineBuilder::new(sample.name).insert_field("value", sample.value);
            for (k, v) in sample.tags.clone() {
                lb = lb.insert_tag(k, v);
            }
            lb.build().to_string().into_bytes()
        };
        let metadata = Metadata {
            r#type: "metric".to_string(),
            size: payload.len() as _,
            metric_meta: Some(MetricMeta {
                start_time: sample.ts.unwrap_or(0),
                end_time: sample.ts.unwrap_or(0),
                key_type: sample
                    .tags
                    .into_iter()
                    .map(|(k, _)| (k, "String".to_string()))
                    .collect(),
            }),
        };
        let req = DataRequest {
            metadata: Some(metadata),
            payload,
        };
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(rpc_client.send_data(req))?;
        Ok(Ok(()))
    }
}

impl profiling::data_export::measurement::Host for DataExportCtx {
    fn export_point(&mut self, mut point: Point) -> wasmtime::Result<Result<(), String>> {
        let Some(rpc_client) = &mut self.rpc_client else {
            return Ok(Ok(()));
        };

        let instance_id = rpc_client
            .instance_id()
            .unwrap_or_else(|_| "unknown".to_string());
        point.tags.push(("instance_id".to_string(), instance_id));

        let payload = {
            let mut lb = LineBuilder::new(point.name);
            for (k, v) in point.tags.clone() {
                lb = lb.insert_tag(k, v);
            }
            for (k, v) in point.fields {
                lb = lb.insert_field(k, v);
            }
            lb.build().to_string().into_bytes()
        };
        let metadata = Metadata {
            r#type: "measurement".to_string(),
            size: payload.len() as _,
            metric_meta: None,
        };
        let req = DataRequest {
            metadata: Some(metadata),
            payload,
        };
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(rpc_client.send_data(req))?;
        Ok(Ok(()))
    }
}

pub fn add_to_linker<T>(
    l: &mut Linker<T>,
    f: impl (Fn(&mut T) -> &mut DataExportCtx) + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    Imports::add_to_linker(l, f)
}
