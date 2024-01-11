use crate::tpls::engine::render_template;
use anyhow::Result;
use serde::{Deserialize, Serialize};

const REPO_TPL: &str = r#"
package repo

import (
	"context"

	"{{app_pkg_path}}/entity"
	"{{app_pkg_path}}/repo/dbal"

	"{{project_name}}/common/tools/filterx"
)

// {{entity_name}}Repo . @GI
type {{entity_name}}Repo struct {
	DBAL *dbal.{{entity_name}}RepoDBAL
}

func New{{entity_name}}Repo() *{{entity_name}}Repo {
	return &{{entity_name}}Repo{
		DBAL: dbal.New{{entity_name}}RepoDBAL(),
	}
}

func (r *{{entity_name}}Repo) Query(ctx context.Context, query filterx.FilteringList, pg *filterx.Page) (entity.{{entity_name}}List, int, error) {
	return r.DBAL.Query(ctx,query,pg)
}

func (r *{{entity_name}}Repo) Count(ctx context.Context, query filterx.FilteringList) (int64, error) {
	return r.DBAL.Count(ctx, query)
}

func (r *{{entity_name}}Repo) QueryOne(ctx context.Context, query filterx.FilteringList) (*entity.{{entity_name}}, error) {
	return r.DBAL.QueryOne(ctx, query)
}

func (r *{{entity_name}}Repo) Create(ctx context.Context, input *entity.{{entity_name}}) (*entity.{{entity_name}}, error) {
	return r.DBAL.Create(ctx,input)
}

func (r *{{entity_name}}Repo) Save(ctx context.Context, input *entity.{{entity_name}}) (*entity.{{entity_name}}, error) {
	return r.DBAL.Save(ctx,input)
}


func (r *{{entity_name}}Repo) Transaction(ctx context.Context, executeFunc func(tx *gorm.DB) error) error {
	return r.DBAL.Transaction(ctx, executeFunc)
}

func (r *{{entity_name}}Repo) UpdateByQuery(ctx context.Context, query filterx.FilteringList, updates map[string]any) error {
	return r.DBAL.UpdateByQuery(ctx, query, updates)
}


func (r *{{entity_name}}Repo) DeleteByQuery(ctx context.Context, query filterx.FilteringList) error {
	return r.DBAL.DeleteByQuery(ctx, query)
}


{{#if has_id}}
func (r *{{entity_name}}Repo) GetByID(ctx context.Context, id int64) (*entity.{{entity_name}}, error) {
	return r.DBAL.GetByID(ctx,id)
}

func (r *{{entity_name}}Repo) GetListByIDs(ctx context.Context, ids []int64) (entity.{{entity_name}}List, error) {
	return r.DBAL.GetListByIDs(ctx,ids)
}

func (r *{{entity_name}}Repo) UpdateByID(ctx context.Context, id int64, updates map[string]any) error {
	return r.DBAL.UpdateByID(ctx,id,updates)
}

func (r *{{entity_name}}Repo) UpdateByIDs(ctx context.Context, ids []int64, updates map[string]any) error {
	return r.DBAL.UpdateByIDs(ctx,ids,updates)
}

func (r *{{entity_name}}Repo) DeleteByID(ctx context.Context, id int64) error {
	return r.DBAL.DeleteByID(ctx,id)
}

func (r *{{entity_name}}Repo) DeleteByIDs(ctx context.Context, ids []int64) error {
	return r.DBAL.DeleteByIDs(ctx,ids)
}
{{/if}}

"#;

const REPO_DBAL_TPL: &str = r#"
package dbal

import (
	"context"

	"gorm.io/gorm"

	"{{app_pkg_path}}/config"
	"{{app_pkg_path}}/entity"
	"{{app_pkg_path}}/repo/dbal/converter"
	"{{app_pkg_path}}/repo/dbal/dao"
	"{{app_pkg_path}}/repo/dbal/do"

	"{{project_name}}/common/lib/db"
	"{{project_name}}/common/tools/filterx"
)

// {{entity_name}}RepoDBAL .
type {{entity_name}}RepoDBAL struct {
	Dao *dao.{{entity_name}}Dao
}

func New{{entity_name}}RepoDBAL() *{{entity_name}}RepoDBAL {
	return &{{entity_name}}RepoDBAL{
		Dao: dao.New{{entity_name}}Dao(),
	}
}

func (impl *{{entity_name}}RepoDBAL) NewReadSession(ctx context.Context) *gorm.DB {
	return impl.NewCreateSession(ctx)
}

func (impl *{{entity_name}}RepoDBAL) NewUpdateSession(ctx context.Context) *gorm.DB {
	return impl.NewCreateSession(ctx)
}

func (impl *{{entity_name}}RepoDBAL) NewCreateSession(ctx context.Context) *gorm.DB {
	session := config.GetDB().NewSession(ctx)
	// todo 是否分库和分表，规则等
	session = session.Table("{{table_name}}")
	return session
}

func (impl *{{entity_name}}RepoDBAL) NewTransactionSession(ctx context.Context) *gorm.DB {
	session := config.GetDB().NewSession(ctx)
	return session
}

func (impl *{{entity_name}}RepoDBAL) Query(ctx context.Context, query filterx.FilteringList, pg *filterx.Page) (entity.{{entity_name}}List, int, error) {
	session := impl.NewReadSession(ctx)
	session, err := query.GormOption(session)
	if err != nil {
		return nil, 0, err
	}
	session, noCount := filterx.PageGormOption(session, pg)
	var (
		doList do.{{entity_name}}DoList
		count  int
	)
	if noCount {
		doList, err = impl.Dao.FindAll(session)
	} else {
		doList, count, err = impl.Dao.FindPage(session)
	}
	if err != nil {
		return nil, 0, err
	}
	return converter.To{{entity_name}}List(doList), count, nil
}

func (impl *{{entity_name}}RepoDBAL) Count(ctx context.Context, query filterx.FilteringList) (int64, error) {
	session := impl.NewReadSession(ctx)
	session, err := query.GormOption(session)
	if err != nil {
		return 0, err
	}
	return impl.Dao.Count(session)
}

func (impl *{{entity_name}}RepoDBAL) QueryOne(ctx context.Context, query filterx.FilteringList) (*entity.{{entity_name}}, error) {
	session := impl.NewReadSession(ctx)
	session, err := query.GormOption(session)
	if err != nil {
		return nil, err
	}
	_do, err := impl.Dao.Get(session)
	if err != nil {
		return nil, err
	}
	return converter.To{{entity_name}}Entity(_do), nil
}

func (impl *{{entity_name}}RepoDBAL) Create(ctx context.Context, input *entity.{{entity_name}}) (*entity.{{entity_name}}, error) {
	session := impl.NewCreateSession(ctx)
	_do := converter.From{{entity_name}}Entity(input)
	err := impl.Dao.Create(session, _do)
	if err != nil {
		return nil, err
	}
	output := converter.To{{entity_name}}Entity(_do)
	return output, err
}

func (impl *{{entity_name}}RepoDBAL) Save(ctx context.Context, input *entity.{{entity_name}}) (*entity.{{entity_name}}, error) {
	session := impl.NewCreateSession(ctx)
	_do := converter.From{{entity_name}}Entity(input)
	err := impl.Dao.Save(session, _do)
	if err != nil {
		return nil, err
	}
	output := converter.To{{entity_name}}Entity(_do)
	return output, err
}

func (impl *{{entity_name}}RepoDBAL) Transaction(ctx context.Context, executeFunc func(tx *gorm.DB) error) (err error) {
	// 注意使用的场景（分库分表情况慎用）
	session := impl.NewTransactionSession(ctx)
	err = session.Transaction(executeFunc)
	return err
}

func (impl *{{entity_name}}RepoDBAL) DemoTransactionWithFunc(ctx context.Context, withFunList []func() error) (err error) {
	// 这是例子 请针对性业务定制 注意使用的场景
	session := impl.NewTransactionSession(ctx)
	err = session.Transaction(func(tx *gorm.DB) error {
		//do something
		for _, fun := range withFunList {
			err = fun()
			if err != nil {
				return err
			}
		}
		//do something
		return nil
	})
	return err
}

func (impl *{{entity_name}}RepoDBAL) UpdateByQuery(ctx context.Context, query filterx.FilteringList, updates map[string]any) error {
	session := impl.NewUpdateSession(ctx)
	session, err := query.GormOption(session)
	if err != nil {
		return err
	}
	err = impl.Dao.Update(session, updates)
	if err != nil {
		return err
	}
	return err
}

func (impl *{{entity_name}}RepoDBAL) DeleteByQuery(ctx context.Context, query filterx.FilteringList) error {
	session := impl.NewUpdateSession(ctx)
	session, err := query.GormOption(session)
	if err != nil {
		return err
	}
	err = impl.Dao.Delete(session)
	if err != nil {
		return err
	}
	return err
}

{{#if has_id}}

func (impl *{{entity_name}}RepoDBAL) GetByID(ctx context.Context, id int64) (*entity.{{entity_name}}, error) {
	session := impl.NewReadSession(ctx)
	session = session.Where("id = ?",id)
	_do, err := impl.Dao.Get(session)
	if err != nil {
		return nil, err
	}
	return converter.To{{entity_name}}Entity(_do), nil
}

func (impl *{{entity_name}}RepoDBAL) GetListByIDs(ctx context.Context, ids []int64) (entity.{{entity_name}}List, error) {
	session := impl.NewReadSession(ctx)
	session = session.Where("id in ?", ids)
	_doList, err := impl.Dao.FindAll(session)
	if err != nil {
		return nil, err
	}
	return converter.To{{entity_name}}List(_doList), nil
}

func (impl *{{entity_name}}RepoDBAL) UpdateByID(ctx context.Context, id int64, updates map[string]any) error {
	session := impl.NewUpdateSession(ctx)
	session = session.Where("id = ?",id)
	err := impl.Dao.Update(session, updates)
	if err != nil {
		return err
	}
	return err
}

func (impl *{{entity_name}}RepoDBAL) UpdateByIDs(ctx context.Context, ids []int64, updates map[string]any) error {
	session := impl.NewUpdateSession(ctx)
	session = session.Where("id in ?",ids)
	err := impl.Dao.Update(session, updates)
	if err != nil {
		return err
	}
	return err
}

func (impl *{{entity_name}}RepoDBAL) DeleteByID(ctx context.Context, id int64) error {
	session := impl.NewUpdateSession(ctx)
	session = session.Where("id = ?",id)
	err := impl.Dao.Delete(session)
	if err != nil {
		return err
	}
	return err
}

func (impl *{{entity_name}}RepoDBAL) DeleteByIDs(ctx context.Context, ids []int64) error {
	session := impl.NewUpdateSession(ctx)
	session = session.Where("id in ?", ids)
	err := impl.Dao.Delete(session)
	if err != nil {
		return err
	}
	return err
}

{{/if}}
"#;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Repo {
    pub project_name: String,
    pub app_pkg_path: String,
    pub entity_name: String,
    pub table_name: String,
    pub has_id: bool,
}

impl Repo {
    pub fn execute(&self) -> Result<String> {
        render_template(REPO_TPL, self)
    }

    pub fn execute_impl(&self) -> Result<String> {
        render_template(REPO_DBAL_TPL, self)
    }
}
