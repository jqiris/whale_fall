use std::{borrow::Borrow, collections::HashMap, rc::Rc};

use gosyn::ast::{self, *};
use regex::Regex;

use crate::common::{
    file::path_str,
    str::{find_string_sub_match, is_first_lowwercase, is_first_uppercase},
};
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum XType {
    XTypeNone = -1,
    XTypeBasic = 0,
    XTypeStruct = 1,
    XTypeSlice = 2,
    XTypeMap = 3,
    XTypeTime = 4,
}
impl Default for XType {
    fn default() -> Self {
        XType::XTypeNone
    }
}
#[derive(Debug, Default, Clone)]
pub struct XArg {
    pub name: String,
    pub xtype: String,
}

#[derive(Debug, Default, Clone)]
pub struct XMethod {
    pub impl_name: String,
    pub name: String,
    pub params: Vec<XArg>,
    pub results: Vec<XArg>,
    pub comment: String,
    pub sort: i32,
    pub http_rule: String,
    pub http_method: String,
}

#[derive(Debug, Default, Clone)]
pub struct TagDesc {
    pub json: Option<TagItem>,
    pub pb: Option<TagItem>,
    pub db: Option<TagItem>,
    pub validate: Option<TagItem>,
}

#[derive(Debug, Default, Clone)]
pub struct TagItem {
    pub name: String,
    pub txt: String,
    pub opts: HashMap<String, String>,
}
#[derive(Debug, Default, Clone)]
pub struct XField {
    pub name: String,
    pub xtype: String,
    pub stype: XType,
    pub idx: i32,
    pub tag: String,
    pub comment: String,
}

impl XField {
    pub fn get_tag(&self, tag: &str) -> Option<TagItem> {
        let re = Regex::new(&(tag.to_owned() + r#":"(\S+)""#)).unwrap();
        if let Some(rs) = re.captures(&self.tag) {
            let txt = rs.get(1).unwrap().as_str().to_owned();
            if txt == "-" {
                return Some(TagItem {
                    name: "-".to_owned(),
                    txt: "-".to_owned(),
                    opts: HashMap::new(),
                });
            }
            let mut it = TagItem {
                name: txt.clone(),
                txt: txt.clone(),
                opts: HashMap::new(),
            };
            if txt.contains(":") || txt.contains(";") {
                it.name = self.name.clone();
                let tmp: Vec<&str> = txt.split(';').collect();
                for (idx, s) in tmp.iter().enumerate() {
                    if idx == 0 {
                        it.name = s.to_string();
                    }
                    let r: Vec<&str> = s.split(':').collect();
                    if r.len() == 2 {
                        it.opts.insert(r[0].to_owned(), r[1].to_owned());
                        if r[0] == "column" {
                            it.name = r[1].to_owned();
                        }
                    } else {
                        it.opts.insert(r[0].to_owned(), "".to_owned());
                    }
                }
            }
            return Some(it);
        }
        None
    }
}
#[derive(Debug, Default, Clone)]
pub struct XST {
    pub gi_name: String,
    pub gi: bool,
    pub impl_inf: String,
    pub imports: Vec<String>,
    pub file: String,
    pub name: String,
    pub short_name: String,
    pub mpoint: bool,
    pub cst: Vec<String>,
    pub methods: HashMap<String, XMethod>,
    pub fields: HashMap<String, XField>,
}
#[derive(Debug, Default, Clone)]
pub struct INF {
    pub name: String,
    pub file: String,
    pub imports: Vec<String>,
    pub methods: HashMap<String, XMethod>,
}
#[derive(Debug, Clone)]
pub struct MetaGo {
    pub ast_file: Option<ast::File>,
    pub inf_list: HashMap<String, INF>,      //interface list
    pub st_list: HashMap<String, XST>,       //struct list
    pub ot_list: HashMap<String, XST>,       //other type list
    pub const_list: HashMap<String, String>, //const list
    pub bind_func_maps: HashMap<String, HashMap<String, XMethod>>, //bind func
    pub func_list: HashMap<String, XMethod>, //func
    pub new_func_list: HashMap<String, XMethod>, //new func
}
impl Default for MetaGo {
    fn default() -> Self {
        Self {
            ast_file: None,
            inf_list: Default::default(),
            st_list: Default::default(),
            ot_list: Default::default(),
            const_list: Default::default(),
            bind_func_maps: Default::default(),
            func_list: Default::default(),
            new_func_list: Default::default(),
        }
    }
}

impl MetaGo {
    pub fn merge(&mut self, go: &MetaGo) {
        if let Some(_) = go.ast_file {
            //仅支持文件合并
            for (name, inf) in go.inf_list.iter() {
                self.inf_list.entry(name.clone()).or_insert(inf.clone());
            }
            for (name, st) in go.st_list.iter() {
                self.st_list.entry(name.clone()).or_insert(st.clone());
            }
            for (name, ot) in go.ot_list.iter() {
                self.ot_list.entry(name.clone()).or_insert(ot.clone());
            }
            for (name, cst) in go.const_list.iter() {
                self.const_list.entry(name.clone()).or_insert(cst.clone());
            }
            for (name, func) in go.func_list.iter() {
                self.func_list.entry(name.clone()).or_insert(func.clone());
            }
            for (name, func) in go.new_func_list.iter() {
                self.new_func_list
                    .entry(name.clone())
                    .or_insert(func.clone());
            }
            for (name, func_maps) in go.bind_func_maps.iter() {
                if self.bind_func_maps.contains_key(name) {
                    let self_func_maps = self.bind_func_maps.get_mut(name).unwrap();
                    for (mname, mth) in func_maps {
                        self_func_maps.entry(mname.clone()).or_insert(mth.clone());
                    }
                } else {
                    self.bind_func_maps.insert(name.clone(), func_maps.clone());
                }
            }
        }
    }
    pub fn load_binds(&mut self) {
        for (sname, sts) in self.st_list.iter_mut() {
            let mut smths: HashMap<String, XMethod> = HashMap::new();
            let mut point = false;
            if let Some(mths) = self.bind_func_maps.get(sname) {
                for (mname, mth) in mths {
                    smths.insert(mname.clone(), mth.clone());
                }
            }
            let xsname = format!("*{}", sname);
            if let Some(mths) = self.bind_func_maps.get(&xsname) {
                point = true;
                for (mname, mth) in mths {
                    smths.insert(mname.clone(), mth.clone());
                }
            }
            for cs in sts.cst.iter() {
                if let Some(cmths) = self.bind_func_maps.get(cs) {
                    for (mname, mth) in cmths {
                        smths.insert(mname.clone(), mth.clone());
                    }
                    let xcs = format!("*{}", cs);
                    if let Some(cmths) = self.bind_func_maps.get(&xcs) {
                        point = true;
                        for (mname, mth) in cmths {
                            smths.insert(mname.clone(), mth.clone());
                        }
                    }
                }
            }
            if smths.len() < 1 {
                continue;
            }
            sts.mpoint = point;
            sts.methods = smths.clone();
            for (_, mth) in smths {
                sts.short_name = mth.impl_name.clone();
                break;
            }
        }
    }
}

pub fn go_map_type_str(arg: &MapType) -> String {
    let mut key = "".to_string();
    match arg.key.borrow() {
        Expression::Ident(x) => {
            key = x.name.clone();
        }
        _ => {}
    }
    let (value, _) = go_type_str(&arg.val);
    format!("map[{}]{}", key, value)
}

pub fn go_array_type_str(arg: &ArrayType) -> String {
    let (value, _) = go_type_str(&arg.typ);
    format!("[]{}", value)
}

pub fn go_slice_type_str(arg: &SliceType) -> String {
    let (value, _) = go_type_str(&arg.typ);
    format!("[]{}", value)
}

pub fn go_type_str(arg: &Expression) -> (String, XType) {
    match arg {
        Expression::Selector(x) => {
            let (point, _) = go_type_str(&x.x);
            let name = format!("{}.{}", point, x.sel.name);
            (name, XType::XTypeStruct)
        }
        Expression::Star(x) => match x.right.borrow() {
            Expression::Selector(_type) => {
                let (point, _) = go_type_str(&_type.x);
                let name = format!("*{}.{}", point, _type.sel.name);
                (name, XType::XTypeStruct)
            }
            Expression::Ident(_type) => {
                if is_first_uppercase(&_type.name) {
                    return (format!("*{}", _type.name), XType::XTypeStruct);
                }
                (format!("*{}", _type.name), XType::XTypeBasic)
            }
            _ => ("".to_string(), XType::XTypeNone),
        },
        Expression::Ident(x) => {
            if is_first_uppercase(&x.name) {
                return (x.name.clone(), XType::XTypeStruct);
            }
            (x.name.clone(), XType::XTypeBasic)
        }
        Expression::TypeMap(x) => (go_map_type_str(x), XType::XTypeMap),
        Expression::TypeArray(x) => (go_array_type_str(x), XType::XTypeSlice),
        Expression::TypeInterface(_) => ("interface{}".to_string(), XType::XTypeBasic),
        Expression::TypePointer(x) => {
            let (point, xtype) = go_type_str(&x.typ);
            (format!("*{}", point), xtype)
        }
        Expression::Call(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
        Expression::Index(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
        Expression::IndexList(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
        Expression::Slice(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
        Expression::FuncLit(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
        Expression::Ellipsis(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
        Expression::BasicLit(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
        Expression::Range(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
        Expression::Paren(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
        Expression::TypeAssert(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
        Expression::CompositeLit(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
        Expression::List(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
        Expression::Operation(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
        Expression::TypeSlice(x) => (go_slice_type_str(x), XType::XTypeSlice),
        Expression::TypeFunction(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
        Expression::TypeStruct(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
        Expression::TypeChannel(_x) => {
            // println!("{:?}", x);
            ("".to_string(), XType::XTypeBasic)
        }
    }
}

pub fn go_func_args(xtype: &FuncType) -> (Vec<XArg>, Vec<XArg>) {
    let mut params = Vec::new();
    let mut results = Vec::new();
    for (idx, param) in xtype.params.list.iter().enumerate() {
        let mut arg = XArg::default();
        (arg.xtype, _) = go_type_str(&param.typ);
        if param.name.len() > 0 {
            arg.name = param.name[0].name.clone();
        } else {
            if arg.xtype.contains("Context") {
                arg.name = "ctx".to_string();
            } else if arg.xtype.contains("Request") {
                arg.name = "req".to_string();
            } else {
                arg.name = format!("arg{}", idx);
            }
        }
        params.push(arg);
    }
    for ret in &xtype.result.list {
        let mut arg = XArg::default();
        (arg.xtype, _) = go_type_str(&ret.typ);
        if ret.name.len() > 0 {
            arg.name = ret.name[0].name.clone();
        }
        results.push(arg);
    }
    (params, results)
}

pub fn go_interface_func(xtype: InterfaceType) -> HashMap<String, XMethod> {
    let mut methods = HashMap::new();
    for (idx, mt) in xtype.methods.list.iter().enumerate() {
        let comment: String = go_merge_comment(&mt.comments);
        let mt_type = mt.typ.clone();
        match mt_type {
            gosyn::ast::Expression::TypeInterface(data) => {
                let mths = go_interface_func(data);
                for (k, v) in mths {
                    if methods.contains_key(k.as_str()) {
                        continue;
                    }
                    methods.insert(k, v);
                }
            }
            gosyn::ast::Expression::TypeFunction(data) => {
                let (params, results) = go_func_args(&data);
                let method = XMethod {
                    name: mt.name[0].name.clone(),
                    impl_name: "impl".to_string(),
                    params,
                    results,
                    comment,
                    sort: idx as i32,
                    http_rule: "".to_string(),
                    http_method: "".to_string(),
                };
                methods.insert(method.name.clone(), method);
            }
            _ => {}
        }
    }
    methods
}

pub fn go_struct_field(xtype: &StructType) -> (HashMap<String, XField>, Vec<String>) {
    let mut fields = HashMap::new();
    let mut child = Vec::new();
    for (idx, fe) in xtype.fields.iter().enumerate() {
        if fe.name.len() > 0 {
            let name = fe.name[0].name.clone();
            if is_first_lowwercase(&name) {
                continue;
            }
            let (xtype, stype) = go_type_str(&fe.typ);
            let mut xf = XField {
                name,
                xtype,
                stype,
                idx: idx as i32,
                tag: "".to_string(),
                comment: "".to_string(),
            };
            if let Some(tag) = &fe.tag {
                xf.tag = tag.value.clone();
            }
            xf.comment = go_merge_comment(&fe.comments);
            fields.insert(xf.name.clone(), xf);
        } else {
            let name: Option<String> = match &fe.typ {
                Expression::TypeMap(_) => Some(fe.name[0].name.clone()),
                Expression::TypeArray(_) => Some(fe.name[0].name.clone()),
                Expression::TypeSlice(_) => Some(fe.name[0].name.clone()),
                Expression::TypeFunction(_) => Some(fe.name[0].name.clone()),
                Expression::TypeStruct(xt) => {
                    let (child_fields, child_child) = go_struct_field(xt);
                    child.extend(child_child);
                    for (k, v) in child_fields {
                        if fields.contains_key(k.as_str()) {
                            continue;
                        }
                        fields.insert(k, v);
                    }
                    None
                }
                Expression::TypeChannel(_) => Some(fe.name[0].name.clone()),
                Expression::TypePointer(_) => Some(fe.name[0].name.clone()),
                Expression::TypeInterface(_) => Some(fe.name[0].name.clone()),
                _ => None,
            };
            if let Some(name) = name {
                child.push(name);
            }
        }
    }
    (fields, child)
}

pub fn go_merge_comment(docs: &Vec<Rc<Comment>>) -> String {
    docs.iter()
        .map(|comment| {
            let mut text = comment.text.clone().replace("//", "");
            text = text.strip_prefix(" ").unwrap_or(&text).to_string();
            text
        })
        .collect::<Vec<String>>()
        .join("\n")
}

impl From<ast::File> for MetaGo {
    fn from(ast_file: ast::File) -> Self {
        // let path_string = ast_file.path.to_string_lossy().to_string();
        // if path_string.contains("mail.go") {
        //     println!("{}", path_string);
        // }
        let mut meta_go = MetaGo {
            ast_file: Some(ast_file.clone()),
            inf_list: HashMap::new(),
            st_list: HashMap::new(),
            ot_list: HashMap::new(),
            const_list: HashMap::new(),
            bind_func_maps: HashMap::new(),
            func_list: HashMap::new(),
            new_func_list: HashMap::new(),
        };
        let re_impl = Regex::new(r"@IMPL\[([\w|.]+)]").unwrap();
        let re_di = Regex::new(r"@DI\[([\w|.]+)]").unwrap();
        let mut imports = Vec::new();
        for import in ast_file.imports {
            imports.push(import.path.value);
        }
        for decl in ast_file.decl {
            match decl {
                ast::Declaration::Function(x) => {
                    let comment = go_merge_comment(&x.docs);
                    if let Some(recv) = x.recv {
                        let (mut bind_name, mut impl_name) = ("".to_string(), "impl".to_string());
                        for field in recv.list {
                            (bind_name, _) = go_type_str(&field.typ);
                            if field.name.len() > 0 {
                                impl_name = field.name[0].name.clone();
                            }
                        }
                        let (params, results) = go_func_args(&x.typ);
                        let mtd = XMethod {
                            impl_name,
                            name: x.name.name.clone(),
                            params,
                            results,
                            comment,
                            ..Default::default()
                        };
                        if !meta_go.bind_func_maps.contains_key(&bind_name) {
                            meta_go
                                .bind_func_maps
                                .insert(bind_name.clone(), HashMap::new());
                        }
                        meta_go
                            .bind_func_maps
                            .get_mut(&bind_name)
                            .unwrap()
                            .insert(mtd.name.clone(), mtd);
                    } else {
                        let (params, results) = go_func_args(&x.typ);
                        let mtd = XMethod {
                            name: x.name.name.clone(),
                            params,
                            results,
                            comment,
                            ..Default::default()
                        };
                        meta_go.func_list.insert(mtd.name.clone(), mtd.clone());
                        if x.name.name.starts_with("New") {
                            let func_name = x.name.name.strip_prefix("New").unwrap();
                            meta_go.new_func_list.insert(func_name.to_string(), mtd);
                        }
                    }
                }
                ast::Declaration::Type(x) => {
                    for spec in x.specs {
                        let (mut used, mut impl_inf, mut gi_name, mut gi) =
                            (true, "".to_string(), "".to_string(), false);
                        if spec.docs.len() > 0 {
                            for comment in spec.docs {
                                if comment.text.contains("@IGNORE") {
                                    used = false;
                                }
                                if comment.text.contains("@IMPL[") {
                                    let rs = find_string_sub_match(&re_impl, &comment.text);
                                    if rs.len() > 1 {
                                        impl_inf = rs[1].clone();
                                    }
                                }
                                if comment.text.contains("@GI") {
                                    gi = true;
                                    let rs = find_string_sub_match(&re_di, &comment.text);
                                    if rs.len() > 1 {
                                        gi_name = rs[1].clone();
                                    }
                                }
                            }
                        }
                        let name = spec.name.name;
                        match spec.typ {
                            ast::Expression::TypeStruct(xt) => {
                                let (fields, child) = go_struct_field(&xt);
                                let xst = XST {
                                    gi_name: gi_name.to_string(),
                                    gi,
                                    impl_inf: impl_inf.to_string(),
                                    imports: imports.clone(),
                                    file: ast_file.path.to_str().unwrap().to_string(),
                                    name: name.clone(),
                                    short_name: "".to_string(),
                                    mpoint: false,
                                    cst: child,
                                    methods: HashMap::new(),
                                    fields,
                                };
                                if !used {
                                    meta_go.ot_list.insert(name, xst);
                                    continue;
                                }
                                meta_go.st_list.insert(name, xst);
                            }
                            ast::Expression::TypeInterface(xt) => {
                                if !used {
                                    continue;
                                }
                                let inf = INF {
                                    name: name.clone(),
                                    file: path_str(&ast_file.path),
                                    imports: imports.clone(),
                                    methods: go_interface_func(xt),
                                };
                                meta_go.inf_list.insert(name, inf);
                            }
                            _ => {}
                        }
                    }
                }
                ast::Declaration::Const(_) => {}
                ast::Declaration::Variable(_) => {}
            }
        }
        meta_go
    }
}
