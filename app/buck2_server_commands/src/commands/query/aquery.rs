/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::io::Write;

use async_trait::async_trait;
use buck2_build_api::actions::query::ActionQueryNode;
use buck2_build_api::query::oneshot::QUERY_FRONTEND;
use buck2_common::dice::cells::HasCellResolver;
use buck2_error::Context;
use buck2_query::query::syntax::simple::eval::values::QueryEvaluationResult;
use buck2_server_ctx::ctx::ServerCommandContextTrait;
use buck2_server_ctx::partial_result_dispatcher::PartialResultDispatcher;
use buck2_server_ctx::pattern::global_cfg_options_from_client_context;
use buck2_server_ctx::template::run_server_command;
use buck2_server_ctx::template::ServerCommandTemplate;
use dice::DiceTransaction;

use crate::commands::query::printer::QueryResultPrinter;
use crate::commands::query::printer::ShouldPrintProviders;
use crate::commands::query::query_target_ext::QueryCommandTarget;

impl QueryCommandTarget for ActionQueryNode {
    fn call_stack(&self) -> Option<String> {
        None
    }

    fn attr_to_string_alternate(&self, attr: &Self::Attr<'_>) -> String {
        format!("{:#}", attr)
    }

    fn attr_serialize<S: serde::Serializer>(
        &self,
        attr: &Self::Attr<'_>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(attr, serializer)
    }
}

pub(crate) async fn aquery_command(
    ctx: &dyn ServerCommandContextTrait,
    partial_result_dispatcher: PartialResultDispatcher<buck2_cli_proto::StdoutBytes>,
    req: buck2_cli_proto::AqueryRequest,
) -> anyhow::Result<buck2_cli_proto::AqueryResponse> {
    run_server_command(AqueryServerCommand { req }, ctx, partial_result_dispatcher).await
}

struct AqueryServerCommand {
    req: buck2_cli_proto::AqueryRequest,
}

#[async_trait]
impl ServerCommandTemplate for AqueryServerCommand {
    type StartEvent = buck2_data::AqueryCommandStart;
    type EndEvent = buck2_data::AqueryCommandEnd;
    type Response = buck2_cli_proto::AqueryResponse;
    type PartialResult = buck2_cli_proto::StdoutBytes;

    async fn command(
        &self,
        server_ctx: &dyn ServerCommandContextTrait,
        mut partial_result_dispatcher: PartialResultDispatcher<Self::PartialResult>,
        ctx: DiceTransaction,
    ) -> anyhow::Result<Self::Response> {
        aquery(
            server_ctx,
            partial_result_dispatcher.as_writer(),
            ctx,
            &self.req,
        )
        .await
    }

    fn is_success(&self, _: &Self::Response) -> bool {
        true
    }
}

async fn aquery(
    server_ctx: &dyn ServerCommandContextTrait,
    mut stdout: impl Write,
    mut ctx: DiceTransaction,
    request: &buck2_cli_proto::AqueryRequest,
) -> anyhow::Result<buck2_cli_proto::AqueryResponse> {
    let cell_resolver = ctx.get_cell_resolver().await?;

    let output_configuration = QueryResultPrinter::from_request_options(
        &cell_resolver,
        &request.output_attributes,
        request.unstable_output_format,
    )?;

    let buck2_cli_proto::AqueryRequest {
        query,
        query_args,
        context,
        ..
    } = request;

    let client_ctx = context.as_ref().internal_error("No client context")?;
    let global_cfg_options =
        global_cfg_options_from_client_context(client_ctx, server_ctx, &mut ctx).await?;

    let query_result = QUERY_FRONTEND
        .get()?
        .eval_aquery(
            &mut ctx,
            server_ctx.working_dir(),
            query,
            query_args,
            global_cfg_options,
        )
        .await?;

    match query_result {
        QueryEvaluationResult::Single(targets) => {
            output_configuration
                .print_single_output(&mut stdout, targets, false, ShouldPrintProviders::No)
                .await?
        }
        QueryEvaluationResult::Multiple(results) => {
            output_configuration
                .print_multi_output(&mut stdout, results, false, ShouldPrintProviders::No)
                .await?
        }
    };
    Ok(buck2_cli_proto::AqueryResponse {})
}
