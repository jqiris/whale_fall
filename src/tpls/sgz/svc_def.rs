use crate::tpls::engine::render_template;
use anyhow::Result;
use serde::{Deserialize, Serialize};

const SVC_TPL: &str = r#"
package service

import (
	"context"

	"{{package}}/internal/model/do"
	"{{package}}/internal/model/entity"
)

type (
	I{{entity_name}} interface {
		// Find 查询数据
		Find(ctx context.Context, in *do.{{entity_name}}ListInput) (out []*entity.{{entity_name}}, err error)
		// FindOne 查询活动数据
		FindOne(ctx context.Context, in *do.{{entity_name}}ListInput) (out *entity.{{entity_name}}, err error)
		// List 分页读取
		List(ctx context.Context, in *do.{{entity_name}}ListInput) (out *do.{{entity_name}}ListOutput, err error)
		// Add 新增
		Add(ctx context.Context, in *do.{{entity_name}}) (lastInsertId int64, err error)
		// Edit 编辑
		Edit(ctx context.Context, in *do.{{entity_name}}) (affected int64, err error)
		// Remove 删除多条记录模式
		Remove(ctx context.Context, id any) (affected int64, err error)
	}
)

var (
	local{{entity_name}} I{{entity_name}}
)

func {{entity_name}}() I{{entity_name}} {
	if local{{entity_name}} == nil {
		panic("local{{entity_name}} is nil")
	}
	return local{{entity_name}}
}

func Register{{entity_name}}(i I{{entity_name}}) {
	local{{entity_name}} = i
}
"#;

#[derive(Serialize, Deserialize, Default)]
pub struct Svc {
    pub package: String,
    pub entity_name: String,
}

impl Svc {
    pub fn execute(&self) -> Result<String> {
        render_template(SVC_TPL, self)
    }
}
