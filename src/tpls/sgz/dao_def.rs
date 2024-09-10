use crate::tpls::engine::render_template;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::EntityField;
const DAO_TPL: &str = r#"
package internal

import (
	"context"
	"database/sql"
	"math"

	"{{package}}/internal/ml"
	"{{package}}/internal/model/do"
	"{{package}}/internal/model/entity"

	"github.com/gogf/gf/v2/database/gdb"
	"github.com/gogf/gf/v2/frame/g"
	"github.com/gogf/gf/v2/util/gconv"
)

// {{entity_name}}Dao is the data access object for table trade_order_return_item.
type {{entity_name}}Dao struct {
	table   string            // table is the underlying table name of the DAO.
	group   string            // group is the database configuration group name of current DAO.
	columns {{entity_name}}Columns // columns contains all the column names of Table for convenient usage.
}

// {{entity_name}}Columns defines and stores column names for table trade_order_return_item.
type {{entity_name}}Columns struct {
{{#each fields}}
    {{name}}  string // {{comment}}
{{/each}}
    PrimaryKey string // 主键
}

// {{entity_name}}Columns holds the columns for table trade_order_return_item.
var {{entity_sname}}Columns = {{entity_name}}Columns{
{{#each fields}}
    {{name}}: "{{sname}}",
{{/each}}
	PrimaryKey: "{{prikey}}",
}

// New{{entity_name}}Dao creates and returns a new DAO object for table data access.
func New{{entity_name}}Dao() *{{entity_name}}Dao {
	return &{{entity_name}}Dao{
		group:   "{{group}}",
		table:   "{{table}}",
		columns: {{entity_sname}}Columns,
	}
}

// DB retrieves and returns the underlying raw database management object of current DAO.
func (dao *{{entity_name}}Dao) DB() gdb.DB {
	return g.DB(dao.group)
}

// Table returns the table name of current dao.
func (dao *{{entity_name}}Dao) Table() string {
	return dao.table
}

// Columns returns all column names of current dao.
func (dao *{{entity_name}}Dao) Columns() {{entity_name}}Columns {
	return dao.columns
}

// Group returns the configuration group name of database of current dao.
func (dao *{{entity_name}}Dao) Group() string {
	return dao.group
}

// Ctx creates and returns the Model for current DAO, It automatically sets the context for current operation.
func (dao *{{entity_name}}Dao) Ctx(ctx context.Context) *gdb.Model {
	return dao.DB().Model(dao.table).Safe().Ctx(ctx)
}

// Transaction wraps the transaction logic using function f.
// It rollbacks the transaction and returns the error from function f if it returns non-nil error.
// It commits the transaction and returns nil if function f returns nil.
//
// Note that, you should not Commit or Rollback the transaction in function f
// as it is automatically handled by this function.
func (dao *{{entity_name}}Dao) Transaction(ctx context.Context, f func(ctx context.Context, tx gdb.TX) error) (err error) {
	return dao.Ctx(ctx).Transaction(ctx, f)
}

// Get 读取一条记录
func (dao *{{entity_name}}Dao) Get(ctx context.Context, id any) (one *entity.{{entity_name}}, err error) {
	var entitys []*entity.{{entity_name}}
	entitys, err = dao.Gets(ctx, id)

	if err != nil {
		return nil, err
	}

	if len(entitys) > 0 {
		one = entitys[0]
	}

	return one, err
}

// Gets 读取多条记录
func (dao *{{entity_name}}Dao) Gets(ctx context.Context, id any) (entitys []*entity.{{entity_name}}, err error) {
	if !g.IsEmpty(id) {
		err = dao.Ctx(ctx).WherePri(id).Scan(&entitys)
	}

	return entitys, err
}

// Find 查询数据
func (dao *{{entity_name}}Dao) Find(ctx context.Context, in *do.{{entity_name}}ListInput) (out []*entity.{{entity_name}}, err error) {
	var (
		m = dao.Ctx(ctx)
	)

	query := m.Where(in.Where).OmitNil()
	query = ml.BuildWhere(query, in.WhereLike, in.WhereExt)

	// 排序
	query = ml.BuildOrder(query, in.Sidx, in.Sort)
	if len(in.Order) > 0 {
		for _, it := range in.Order {
			query = ml.BuildOrder(query, it.Sidx, it.Sort)
		}
	}

	// 对象转换
	if err := query.Scan(&out); err != nil {
		return out, err
	}

	return out, nil
}

// FindOne 查询一条数据
func (dao *{{entity_name}}Dao) FindOne(ctx context.Context, in *do.{{entity_name}}ListInput) (one *entity.{{entity_name}}, err error) {
	in.BaseList.Size = 1

	var entitys []*entity.{{entity_name}}
	entitys, err = dao.Find(ctx, in)

	if err != nil {
		return nil, err
	}

	if len(entitys) > 0 {
		one = entitys[0]
	}

	return one, err
}

// Find 查询字段数据
func (dao *{{entity_name}}Dao) FindFields(ctx context.Context, fieldNamesOrMapStruct interface{}, in *do.{{entity_name}}ListInput) (out gdb.Result, err error) {
	var (
		m = dao.Ctx(ctx)
	)

	query := m.Fields(fieldNamesOrMapStruct).Where(in.Where).OmitNil()
	query = ml.BuildWhere(query, in.WhereLike, in.WhereExt)

	// 排序
	query = ml.BuildOrder(query, in.Sidx, in.Sort)
	if len(in.Order) > 0 {
		for _, it := range in.Order {
			query = ml.BuildOrder(query, it.Sidx, it.Sort)
		}
	}

	out, err = query.All()

	if err != nil {
		return out, err
	}

	return out, nil
}

// FindKey 查询主键数据
func (dao *{{entity_name}}Dao) FindKey(ctx context.Context, in *do.{{entity_name}}ListInput) (out []interface{}, err error) {
	idRes, err := dao.FindFields(ctx, dao.Columns().PrimaryKey, in)

	if err != nil {
		return nil, err
	}

	for _, record := range idRes {
		if !record[dao.Columns().PrimaryKey].IsEmpty() {
			out = append(out, record[dao.Columns().PrimaryKey])
		}
	}

	return out, err
}

// List 分页读取
func (dao *{{entity_name}}Dao) List(ctx context.Context, in *do.{{entity_name}}ListInput) (out *do.{{entity_name}}ListOutput, err error) {
	var (
		m = dao.Ctx(ctx)
	)

	query := m.Where(in.Where).OmitNil()
	query = ml.BuildWhere(query, in.WhereLike, in.WhereExt)

	out = &do.{{entity_name}}ListOutput{}
	out.Page = in.Page
	out.Size = in.Size

	// 查询记录总数
	count, err1 := query.Count()
	if err1 != nil {
		return nil, err1
	}

	out.Records = count
	out.Total = int(math.Ceil(float64(count) / float64(out.Size)))

	// 排序
	query = ml.BuildOrder(query, in.Sidx, in.Sort)
	if len(in.Order) > 0 {
		for _, it := range in.Order {
			query = ml.BuildOrder(query, it.Sidx, it.Sort)
		}
	}

	// 分页
	query = query.Page(in.Page, in.Size)

	// 对象转换
	if err := query.Scan(&out.Items); err != nil {
		return out, err
	}

	return out, nil
}

// Add 新增
func (dao *{{entity_name}}Dao) Add(ctx context.Context, in *do.{{entity_name}}) (lastInsertId int64, err error) {
	data := do.{{entity_name}}{}
	if err = gconv.Scan(in, &data); err != nil {
		return 0, err
	}

	return dao.Ctx(ctx).Data(data).OmitNil().InsertAndGetId()
}

// Edit 编辑
func (dao *{{entity_name}}Dao) Edit(ctx context.Context, id any, in *do.{{entity_name}}) (int64, error) {
	data := do.{{entity_name}}{}
	if err := gconv.Scan(in, &data); err != nil {
		return 0, err
	}

	//FieldsEx(dao.Columns().Id)
	return dao.Ctx(ctx).Data(data).OmitNil().WherePri(id).UpdateAndGetAffected()
}

// EditWhere 根据Where条件编辑
func (dao *{{entity_name}}Dao) EditWhere(ctx context.Context, where *do.{{entity_name}}ListInput, in *do.{{entity_name}}) (int64, error) {
	data := do.{{entity_name}}{}
	if err := gconv.Scan(in, &data); err != nil {
		return 0, err
	}

	query := dao.Ctx(ctx).Data(data).OmitNil().Where(where.Where)
	query = ml.BuildWhere(query, where.WhereLike, where.WhereExt)

	return query.UpdateAndGetAffected()
}

// Save 保存
func (dao *{{entity_name}}Dao) Save(ctx context.Context, in *do.{{entity_name}}) (affected int64, err error) {
	data := do.{{entity_name}}{}
	if err = gconv.Scan(in, &data); err != nil {
		return 0, err
	}

	res, err := dao.Ctx(ctx).Data(data).OmitNil().OnConflict(dao.Columns().PrimaryKey).Save()

	if err != nil {
		return 0, err
	}

	return res.RowsAffected()
}

// Saves 批量保存
func (dao *{{entity_name}}Dao) Saves(ctx context.Context, in []*do.{{entity_name}}) (affected int64, err error) {
	data := []do.{{entity_name}}{}
	if err = gconv.Scan(in, &data); err != nil {
		return 0, err
	}

	res, err := dao.Ctx(ctx).Data(data).OmitNil().OnConflict(dao.Columns().PrimaryKey).Save()

	if err != nil {
		return 0, err
	}

	return res.RowsAffected()
}

// Increment 增加
func (dao *{{entity_name}}Dao) Increment(ctx context.Context, id any, column string, amount interface{}) (sql.Result, error) {
	return dao.Ctx(ctx).WherePri(id).Increment(column, amount)
}

// Decrement 减少
func (dao *{{entity_name}}Dao) Decrement(ctx context.Context, id any, column string, amount interface{}) (sql.Result, error) {
	return dao.Ctx(ctx).WherePri(id).Decrement(column, amount)
}

// Remove 根据主键删除
func (dao *{{entity_name}}Dao) Remove(ctx context.Context, id any) (int64, error) {
	res, err := dao.Ctx(ctx).WherePri(id).Delete()
	if err != nil {
		return 0, err
	}

	return res.RowsAffected()
}

// Remove 根据Where条件删除
func (dao *{{entity_name}}Dao) RemoveWhere(ctx context.Context, where *do.{{entity_name}}ListInput) (int64, error) {
	query := dao.Ctx(ctx).Where(where.Where)
	query = ml.BuildWhere(query, where.WhereLike, where.WhereExt)

	res, err := query.Delete()
	if err != nil {
		return 0, err
	}

	return res.RowsAffected()
}

// Count 查询数据记录
func (dao *{{entity_name}}Dao) Count(ctx context.Context, in *do.{{entity_name}}ListInput) (count int, err error) {
	var (
		m = dao.Ctx(ctx)
	)

	query := m.Where(in.Where).OmitNil()
	query = ml.BuildWhere(query, in.WhereLike, in.WhereExt)

	//记录数
	count, err = query.Count()

	if err != nil {
		return 0, err
	}

	return count, nil
}
"#;

const DAO_SINGLE: &str = r#"
package dao

import (
	"{{package}}/internal/dao/internal"
)

// internal{{entity_name}}Dao is internal type for wrapping internal DAO implements.
type internal{{entity_name}}Dao = *internal.{{entity_name}}Dao

// {{entity_name}}Dao is the data access object for table admin_user_admin.
// You can define custom methods on it to extend its functionality as you wish.
type {{entity_name}}Dao struct {
	internal{{entity_name}}Dao
}

var (
	// {{entity_name}} is globally public accessible object for table admin_user_admin operations.
	{{entity_name}} = {{entity_name}}Dao{
		internal.New{{entity_name}}Dao(),
	}
)

"#;

#[derive(Serialize, Deserialize, Default)]
pub struct Dao {
    pub package: String,
    pub entity_name: String,
    pub entity_sname: String,
    pub group: String,
    pub table: String,
    pub prikey: String,
    pub fields: Vec<EntityField>,
}

impl Dao {
    pub fn execute(&self) -> Result<String> {
        render_template(DAO_TPL, self)
    }

    pub fn execute_single(&self) -> Result<String> {
        render_template(DAO_SINGLE, self)
    }
}
