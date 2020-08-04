use crate::subset::ast::*;

impl Expr {
    pub fn id(&self) -> String {
        match self {
            Expr::Ref(id) => id.to_string(),
            _ => panic!("Error: do not support id"),
        }
    }

    pub fn new_ref(name: &str) -> Expr {
        Expr::Ref(name.to_string())
    }

    pub fn new_str(value: &str) -> Expr {
        Expr::Str(value.to_string())
    }

    pub fn new_ulit_dec(width: u32, value: &str) -> Expr {
        assert!(width > 0, "Error: width must be greater than zero");
        Expr::ULit(width, Radix::Dec, value.to_string())
    }

    pub fn new_ulit_hex(width: u32, value: &str) -> Expr {
        assert!(width > 0, "Error: width must be greater than zero");
        Expr::ULit(width, Radix::Hex, value.to_string())
    }

    pub fn new_ulit_bin(width: u32, value: &str) -> Expr {
        assert!(width > 0, "Error: width must be greater than zero");
        Expr::ULit(width, Radix::Bin, value.to_string())
    }
}

impl Instance {
    pub fn new(id: &str, prim: &str) -> Instance {
        Instance {
            id: id.to_string(),
            prim: prim.to_string(),
            params: Map::new(),
            ports: Map::new(),
        }
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_prim(&mut self, prim: &str) {
        self.prim = prim.to_string();
    }

    pub fn add_param(&mut self, param: &str, value: Expr) {
        self.params.insert(param.to_string(), value);
    }

    pub fn add_param_uint(&mut self, param: &str, value: u32) {
        self.params.insert(
            param.to_string(),
            Expr::new_ulit_dec(32, &value.to_string()),
        );
    }

    pub fn add_param_str(&mut self, param: &str, value: &str) {
        self.params.insert(param.to_string(), Expr::new_str(value));
    }

    pub fn connect(&mut self, port: &str, expr: Expr) {
        self.ports.insert(port.to_string(), expr);
    }

    pub fn connect_ref(&mut self, port: &str, id: &str) {
        self.ports.insert(port.to_string(), Expr::new_ref(id));
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn prim(&self) -> String {
        self.prim.to_string()
    }

    pub fn param_map(&self) -> &Map {
        &self.params
    }

    pub fn port_map(&self) -> &Map {
        &self.ports
    }
}
