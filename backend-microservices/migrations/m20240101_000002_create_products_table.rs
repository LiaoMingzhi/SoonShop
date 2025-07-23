use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建商品分类表
        manager
            .create_table(
                Table::create()
                    .table(ProductCategory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductCategory::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProductCategory::Name).string().not_null())
                    .col(ColumnDef::new(ProductCategory::Description).text())
                    .col(ColumnDef::new(ProductCategory::ParentId).uuid())
                    .col(ColumnDef::new(ProductCategory::SortOrder).integer().default(0))
                    .col(ColumnDef::new(ProductCategory::IsActive).boolean().default(true))
                    .col(
                        ColumnDef::new(ProductCategory::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ProductCategory::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_category_parent")
                            .from(ProductCategory::Table, ProductCategory::ParentId)
                            .to(ProductCategory::Table, ProductCategory::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建商品表
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(ColumnDef::new(Product::Description).text())
                    .col(ColumnDef::new(Product::Sku).string().unique_key())
                    .col(ColumnDef::new(Product::CategoryId).uuid().not_null())
                    .col(ColumnDef::new(Product::ProducerId).uuid().not_null())
                    .col(ColumnDef::new(Product::Price).decimal().not_null())
                    .col(ColumnDef::new(Product::Currency).string().default("USD"))
                    .col(ColumnDef::new(Product::Weight).decimal())
                    .col(ColumnDef::new(Product::Dimensions).json())
                    .col(ColumnDef::new(Product::Images).json())
                    .col(ColumnDef::new(Product::Attributes).json())
                    .col(ColumnDef::new(Product::Status).string().default("active"))
                    .col(ColumnDef::new(Product::IsDigital).boolean().default(false))
                    .col(ColumnDef::new(Product::MinOrderQuantity).integer().default(1))
                    .col(ColumnDef::new(Product::MaxOrderQuantity).integer())
                    .col(ColumnDef::new(Product::Tags).json())
                    .col(
                        ColumnDef::new(Product::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Product::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_category")
                            .from(Product::Table, Product::CategoryId)
                            .to(ProductCategory::Table, ProductCategory::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建商品变体表（用于SKU变体管理）
        manager
            .create_table(
                Table::create()
                    .table(ProductVariant::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductVariant::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProductVariant::ProductId).uuid().not_null())
                    .col(ColumnDef::new(ProductVariant::Sku).string().unique_key())
                    .col(ColumnDef::new(ProductVariant::Name).string())
                    .col(ColumnDef::new(ProductVariant::Price).decimal())
                    .col(ColumnDef::new(ProductVariant::Attributes).json())
                    .col(ColumnDef::new(ProductVariant::Images).json())
                    .col(ColumnDef::new(ProductVariant::Status).string().default("active"))
                    .col(
                        ColumnDef::new(ProductVariant::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ProductVariant::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_product_variant_product")
                            .from(ProductVariant::Table, ProductVariant::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建索引
        manager
            .create_index(
                Index::create()
                    .name("idx_product_category_name")
                    .table(ProductCategory::Table)
                    .col(ProductCategory::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_product_sku")
                    .table(Product::Table)
                    .col(Product::Sku)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_product_category_producer")
                    .table(Product::Table)
                    .col(Product::CategoryId)
                    .col(Product::ProducerId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_product_status")
                    .table(Product::Table)
                    .col(Product::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_product_variant_sku")
                    .table(ProductVariant::Table)
                    .col(ProductVariant::Sku)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductVariant::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ProductCategory::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum ProductCategory {
    Table,
    Id,
    Name,
    Description,
    ParentId,
    SortOrder,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Product {
    Table,
    Id,
    Name,
    Description,
    Sku,
    CategoryId,
    ProducerId,
    Price,
    Currency,
    Weight,
    Dimensions,
    Images,
    Attributes,
    Status,
    IsDigital,
    MinOrderQuantity,
    MaxOrderQuantity,
    Tags,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum ProductVariant {
    Table,
    Id,
    ProductId,
    Sku,
    Name,
    Price,
    Attributes,
    Images,
    Status,
    CreatedAt,
    UpdatedAt,
} 