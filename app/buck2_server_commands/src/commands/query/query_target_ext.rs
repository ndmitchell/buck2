/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use buck2_query::query::environment::AttrFmtOptions;
use buck2_query::query::environment::QueryTarget;

/// Extensions of `QueryTarget` needed in query commands.
pub(crate) trait QueryCommandTarget: QueryTarget {
    fn call_stack(&self) -> Option<String>;

    fn attr_to_string_alternate(&self, _options: AttrFmtOptions, attr: &Self::Attr<'_>) -> String;

    fn attr_serialize<S: serde::Serializer>(
        &self,
        attr: &Self::Attr<'_>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>;
}
