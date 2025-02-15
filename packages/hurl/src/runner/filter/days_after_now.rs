/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2024 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

use chrono::Utc;

use hurl_core::ast::SourceInfo;

use crate::runner::{Error, Number, RunnerError, Value};

pub fn eval_days_after_now(
    value: &Value,
    source_info: SourceInfo,
    assert: bool,
) -> Result<Option<Value>, Error> {
    match value {
        Value::Date(value) => {
            let diff = value.signed_duration_since(Utc::now());
            Ok(Some(Value::Number(Number::Integer(diff.num_days()))))
        }
        v => {
            let inner = RunnerError::FilterInvalidInput(v._type());
            Err(Error::new(source_info, inner, assert))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::runner::filter::eval::eval_filter;
    use chrono::offset::Utc;
    use chrono::Duration;
    use hurl_core::ast::{Filter, FilterValue, Pos, SourceInfo};
    use std::collections::HashMap;

    use super::*;

    #[test]
    pub fn eval_filter_days_after_before_now() {
        let variables = HashMap::new();

        let now = Utc::now();
        assert_eq!(
            eval_filter(
                &Filter {
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                    value: FilterValue::DaysAfterNow,
                },
                &Value::Date(now),
                &variables,
                false,
            )
            .unwrap()
            .unwrap(),
            Value::Number(Number::Integer(0))
        );

        let now_plus_30hours = now + Duration::hours(30);
        assert_eq!(
            eval_filter(
                &Filter {
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                    value: FilterValue::DaysAfterNow,
                },
                &Value::Date(now_plus_30hours),
                &variables,
                false,
            )
            .unwrap()
            .unwrap(),
            Value::Number(Number::Integer(1))
        );
        assert_eq!(
            eval_filter(
                &Filter {
                    source_info: SourceInfo::new(Pos::new(1, 1), Pos::new(1, 1)),
                    value: FilterValue::DaysBeforeNow,
                },
                &Value::Date(now_plus_30hours),
                &variables,
                false,
            )
            .unwrap()
            .unwrap(),
            Value::Number(Number::Integer(-1))
        );
    }
}
