use std::collections::HashMap;

use crate::parser::Expression;

/** 100% self-made engine without Copilot!
 *
 * Okay, not 100% self-made, since partially asked some algorithms from duck.ai */
pub struct Engine {
    alpha_count: u32,
}

impl Engine {
    pub fn new() -> Self {
        Self { alpha_count: 0 }
    }

    /** Generate a never used variable name for alpha substitution */
    fn generate_variable_name(&mut self, original_name: &String) -> String {
        self.alpha_count += 1;
        format!("{}~{}", original_name, self.alpha_count)
        /* TODO: deny ~ as identifyer in the parser */
    }

    /** Test if the expression used a external variable called `name` */
    fn contains_name(expr: &Expression, name: &String) -> bool {
        match expr {
            Expression::Variable(var_name) => var_name == name,
            Expression::Function { param, body } => {
                if param == name {
                    false
                } else {
                    Self::contains_name(body, name)
                }
            }
            Expression::Application { function, arg } => {
                Self::contains_name(function, name) || Self::contains_name(arg, name)
            }
        }
    }

    /** Substitute every occurance of variable `name` in expression `expr` to `value` */
    fn substitute(&mut self, expr: Expression, name: &String, value: &Expression) -> Expression {
        match expr {
            Expression::Variable(var_name) => {
                if var_name == *name {
                    /* this is what we actually mean to do */
                    return value.clone();
                } else {
                    return Expression::Variable(var_name);
                }
            }
            Expression::Function { param, body } => {
                if param == *name {
                    /* no substitution if parameter shadows the variable */
                    return Expression::Function { param, body };
                }
                // TODO: increase performance by merging substutute and contains_name
                // Let substitute return a tuple contains bool of whether name is contained.
                if !Self::contains_name(value, &param) {
                    /* no alpha substitution needed */
                    let body = self.substitute(*body, name, value);
                    return Expression::Function {
                        param,
                        body: Box::new(body),
                    };
                }

                /* first perform alpha substitution */
                let new_param = self.generate_variable_name(&param);
                /* then the two beta ones */
                let body = self.substitute(*body, &param, &Expression::Variable(new_param.clone()));
                let body = self.substitute(body, name, value);
                return Expression::Function {
                    param: new_param,
                    body: Box::new(body),
                };
            }
            Expression::Application { function, arg } => {
                return Expression::Application {
                    function: Box::new(self.substitute(*function, name, value)),
                    arg: Box::new(self.substitute(*arg, name, value)),
                };
            }
        }
    }

    /** Substute for all the variables in the expression. */
    // TODO: substitute variable only when needed, i.e. (x N), where x is been called.
    pub fn put_variables(
        &mut self,
        mut expr: Expression,
        variables: &HashMap<String, Expression>,
    ) -> Expression {
        for (name, value) in variables {
            expr = self.substitute(expr, name, value)
        }
        expr
    }

    pub fn reduct_call_by_name(&mut self, expr: Expression) -> Expression {
        match expr {
            Expression::Application { function, arg } => {
                match self.reduct_call_by_name(*function) {
                    Expression::Function { param, body } => {
                        let expr = self.substitute(*body, &param, &arg);
                        self.reduct_call_by_name(expr)
                    }
                    expr => Expression::Application {
                        function: Box::new(expr),
                        arg,
                    },
                }
            }
            expr => expr,
        }
    }

    pub fn reduct_normal_order(&mut self, expr: Expression) -> Expression {
        match expr {
            Expression::Function { param, body } => Expression::Function {
                param,
                body: Box::new(self.reduct_normal_order(*body)),
            },
            Expression::Application { function, arg } => {
                match self.reduct_normal_order(*function) {
                    Expression::Function { param, body } => {
                        let expr = self.substitute(*body, &param, &arg);
                        self.reduct_normal_order(expr)
                    }
                    expr => Expression::Application {
                        function: Box::new(expr),
                        arg: Box::new(self.reduct_normal_order(*arg)),
                    },
                }
            }
            expr => expr,
        }
    }
}
