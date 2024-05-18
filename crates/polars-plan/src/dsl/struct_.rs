use super::*;

/// Specialized expressions for Struct dtypes.
pub struct StructNameSpace(pub(crate) Expr);

impl StructNameSpace {
    pub fn field_by_index(self, index: i64) -> Expr {
        self.0
            .map_private(FunctionExpr::StructExpr(StructFunction::FieldByIndex(
                index,
            )))
            .with_function_options(|mut options| {
                options.allow_rename = true;
                options
            })
    }

    /// Retrieve one of the fields of this [`StructChunked`] as a new Series.
    pub fn field_by_name(self, name: &str) -> Expr {
        self.0
            .map_private(FunctionExpr::StructExpr(StructFunction::FieldByName(
                ColumnName::from(name),
            )))
            .with_function_options(|mut options| {
                options.allow_rename = true;
                options
            })
    }

    /// Rename the fields of the [`StructChunked`].
    pub fn rename_fields(self, names: Vec<String>) -> Expr {
        self.0
            .map_private(FunctionExpr::StructExpr(StructFunction::RenameFields(
                Arc::from(names),
            )))
    }

    #[cfg(feature = "json")]
    pub fn json_encode(self) -> Expr {
        self.0
            .map_private(FunctionExpr::StructExpr(StructFunction::JsonEncode))
    }

    pub fn with_fields(self, mut fields: Vec<Expr>) -> Expr {
        fields.insert(0, self.0);
        Expr::Function {
            input: fields,
            function: FunctionExpr::StructExpr(StructFunction::WithFields),
            options: FunctionOptions {
                collect_groups: ApplyOptions::ElementWise,
                ..Default::default()
            },
        }
    }
}
