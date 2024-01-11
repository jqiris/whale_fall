use crate::tpls::engine::render_template;
use anyhow::Result;
use serde::{Deserialize, Serialize};
const DAO_TPL: &str = r#"
type {{dao_name}} struct {
}

func New{{dao_name}}() *{{dao_name}} {
  return &{{dao_name}}{}
}

{{if pk_name}}func (dao *{{dao_name}}) GetById(session *gorm.DB, id {{pk_type}}) (*do.{{entity_name}}, error) {
	result := &do.{{entity_name}}{}
	err := session.Where("{{pk_col}} = ?", id).First(result).Error
	if err != nil {
		return nil, errors.Wrapf(err, "{{dao_name}} GetById failed")
	}
	return result, nil
} 
{{end}}

{{if pk_name}}func (dao *{{dao_name}}) GetByIdList(session *gorm.DB, idList []{{pk_type}}) (do.{{entity_list_name}}, error) {
	result := make([]*do.{{entity_name}}, 0)
	if err := session.Where("{{pk_col}} in (?)", idList).Find(&result).Error; err != nil {
		return nil, errors.Wrapf(err, "{{dao_name}} GetByIdList failed")
	}
	return result, nil
} 
{{end}}

func (dao *{{dao_name}}) Create(session *gorm.DB, data *do.{{entity_name}}) error {
	err := session.Create(data).Error
	if err != nil {
		return errors.Wrapf(err, "{{dao_name}} Create failed")
	}
	return nil
}


func (dao *{{dao_name}}) Save(session *gorm.DB, data *do.{{entity_name}}) error {
	err := session.Save(data).Error
	if err != nil {
		return errors.Wrapf(err, "{{dao_name}} Save failed")
	}
	return nil
}


func (dao *{{dao_name}}) CreateBatch(session *gorm.DB, data do.{{entity_list_name}}) error {
	err := session.CreateInBatches(data, len(data)).Error
	if err != nil {
		return errors.Wrapf(err, "{{dao_name}} CreateBatch failed")
	}
	return nil
}

func (dao *{{dao_name}}) Update(session *gorm.DB,updates map[string]any) error {
	err := session.Updates(updates).Error
	if err != nil {
		return errors.Wrapf(err, "{{dao_name}} Update failed")
	}
	return nil
}

func (dao *{{dao_name}}) Delete(session *gorm.DB) error {
	err := session.Delete(&do.{{entity_name}}{}).Error
	if  err != nil {
		return errors.Wrapf(err, "{{dao_name}} Delete failed")
	}
	return nil
}

func (dao *{{dao_name}}) FindPage(session *gorm.DB) (do.{{entity_list_name}}, int, error) {

	result := make([]*do.{{entity_name}}, 0)
	err := session.Find(&result).Error
	if err != nil {
		return nil, 0, errors.Wrapf(err, "{{dao_name}} FindPage failed 数据库错误")
	}
	delete(session.Statement.Clauses, "LIMIT")
	var count int64
	err = session.Count(&count).Error
	if err != nil {
		return nil, 0, errors.Wrapf(err, "{{dao_name}} FindPage failed 数据库错误")
	}
	return result, int(count), nil
}

func (dao *{{dao_name}}) Count(session *gorm.DB) (int64, error) {
	var count int64
	err := session.Count(&count).Error
	if err != nil {
		return 0, errors.Wrapf(err, "{{dao_name}} Count failed 数据库错误")
	}
	return count, nil
}


func (dao *{{dao_name}}) FindAll(session *gorm.DB) (do.{{entity_list_name}}, error) {
	result := make([]*do.{{entity_name}}, 0)
	err := session.Find(&result).Error
	if err != nil {
		return nil, errors.Wrapf(err, "{{dao_name}} FindAll failed 数据库错误")
	}
	return result, nil
}

func (dao *{{dao_name}}) Get(session *gorm.DB) (*do.{{entity_name}}, error) {
	result := &do.{{entity_name}}{}
	err := session.First(result).Error
	if err != nil {
		if err == gorm.ErrRecordNotFound {
			return nil, nil
		}
		return nil, errors.New("记录获取失败")
	}
	return result, nil
}
"#;
#[derive(Serialize, Deserialize, Default)]
pub struct Dao {
    pub entity_name: String,
    pub dao_name: String,
    pub entity_list_name: String,
    pub table_name: String,
    pub pk_name: String,
    pub pk_type: String,
    pub pk_col: String,
}

impl Dao {
    pub fn execute(&self) -> Result<String> {
        render_template(DAO_TPL, self)
    }
}
