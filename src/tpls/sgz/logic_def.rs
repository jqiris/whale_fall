use crate::tpls::engine::render_template;
use anyhow::Result;
use serde::{Deserialize, Serialize};

const LOGIC_TPL: &str = r#"
package {{pkg_name}}

import (
	"context"

	"{{package}}/internal/dao"
	"{{package}}/internal/model/do"
	"{{package}}/internal/model/entity"
	"{{package}}/internal/service"
)

type s{{entity_name}} struct{}

func init() {
	service.Register{{entity_name}}(New{{entity_name}}())
}

func New{{entity_name}}() *s{{entity_name}} {
	return &s{{entity_name}}{}
}

// Find 查询数据
func (s *s{{entity_name}}) Find(ctx context.Context, in *do.{{entity_name}}ListInput) (out []*entity.{{entity_name}}, err error) {
	out, err = dao.{{entity_name}}.Find(ctx, in)

	return out, err
}

// FindOne 查询活动数据
func (s *s{{entity_name}}) FindOne(ctx context.Context, in *do.{{entity_name}}ListInput) (out *entity.{{entity_name}}, err error) {
	list, err := dao.{{entity_name}}.Find(ctx, in)

	if err != nil {
		return nil, err
	}

	if len(list) > 0 {
		return list[0], nil
	}

	return out, err
}

// List 分页读取
func (s *s{{entity_name}}) List(ctx context.Context, in *do.{{entity_name}}ListInput) (out *do.{{entity_name}}ListOutput, err error) {
	out, err = dao.{{entity_name}}.List(ctx, in)

	return out, err
}

// Add 新增
func (s *s{{entity_name}}) Add(ctx context.Context, in *do.{{entity_name}}) (lastInsertId int64, err error) {
	lastInsertId, err = dao.{{entity_name}}.Add(ctx, in)
	if err != nil {
		return 0, err
	}
	return lastInsertId, err
}

// Edit 编辑
func (s *s{{entity_name}}) Edit(ctx context.Context, in *do.{{entity_name}}) (affected int64, err error) {
	_, err = dao.{{entity_name}}.Edit(ctx, in.{{pri_name}}, in)

	if err != nil {
		return 0, err
	}
	return
}

// Remove 删除多条记录模式
func (s *s{{entity_name}}) Remove(ctx context.Context, id any) (affected int64, err error) {
	affected, err = dao.{{entity_name}}.Remove(ctx, id)

	if err != nil {
		return 0, err
	}

	return affected, err
}

// Save 保存
func (s *s{{entity_name}}) Save(ctx context.Context, in *do.{{entity_name}}) (affected int64, err error) {
	return dao.{{entity_name}}.Save(ctx, in)
}

// Count 计数
func (s *s{{entity_name}}) Count(ctx context.Context, in *do.{{entity_name}}ListInput) (count int, err error) {
	return dao.{{entity_name}}.Count(ctx, in)
}
"#;

#[derive(Serialize, Deserialize, Default)]
pub struct Logic {
    pub package: String,
    pub entity_name: String,
    pub pkg_name: String,
    pub pri_name: String,
}

impl Logic {
    pub fn execute(&self) -> Result<String> {
        render_template(LOGIC_TPL, self)
    }
}
