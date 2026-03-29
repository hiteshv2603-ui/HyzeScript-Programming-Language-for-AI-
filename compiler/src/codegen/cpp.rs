// HyzeScript/compiler/src/codegen/cpp.rs

use crate::ir;

pub fn generate(program: &ir::IrProgram) -> Result<String, String> {
    let mut out = String::new();

    out.push_str("#include <iostream>\n");
    out.push_str("#include \"Tensor.hpp\"\n");
    out.push_str("#include \"nn.hpp\"\n\n");
    out.push_str("using namespace std;\n\n");

    for node in &program.nodes {
        match node {
            ir::IrNode::Let { name, ty, expr } => {
                let (cpp_ty, cpp_init) = match ty {
                    ir::IrType::Tensor { dims } => {
                        ("Tensor", format!("Tensor::of({:?})", dims))
                    }
                };

                let expr_code = expr_to_cpp(expr)?;
                out.push_str(&format!("{} {} = {};\n", cpp_ty, name, expr_code));
            }
            ir::IrNode::Call { callee, args } => {
                let mut arg_codes = Vec::new();
                for arg in args {
                    arg_codes.push(expr_to_cpp(arg)?);
                }
                let call = match callee {
                    ir::IrCallee::Tensor => {
                        format!("Tensor::of({})", arg_codes.join(", "))
                    }
                    ir::IrCallee::Linear => {
                        format!("nn::Linear({})", arg_codes.join(", "))
                    }
                    ir::IrCallee::ReLU => {
                        format!("nn::ReLU({})", arg_codes.join(", "))
                    }
                    ir::IrCallee::MatMul => {
                        format!("{} * {}", arg_codes[0], arg_codes[1])
                    }
                };
                out.push_str(&format!("{}\n", call));
            }
            ir::IrNode::Print(expr) => {
                let expr_code = expr_to_cpp(expr)?;
                out.push_str(&format!("std::cout << {} << std::endl;\n", expr_code));
            }
            ir::IrNode::IpuBlock(sub_nodes) => {
                out.push_str("#pragma ipu_region\n{\n");
                for sub in sub_nodes {
                    if let ir::IrNode::Call { callee, args } = sub {
                        let arg_code: Vec<_> = args.iter()
                            .map(|a| expr_to_cpp(a))
                            .collect::<Result<_, _>>()?;
                        out.push_str(&format!("// IPU call: {:?}({})\n", callee, arg_code.join(", ")));
                    }
                }
                out.push_str("}\n");
            }
        }
    }

    Ok(out)
}

fn expr_to_cpp(expr: &ir::IrExpr) -> Result<String, String> {
    Ok(match expr {
        ir::IrExpr::Number(n) => n.to_string(),
        ir::IrExpr::Var(name) => name.clone(),
        ir::IrExpr::Tensor(data) => format!("Tensor::of({:?})", data),
        ir::IrExpr::Call { callee, args } => {
            let arg_code: Vec<String> = args.iter()
                .map(|a| expr_to_cpp(a))
                .collect::<Result<_, _>>()?;
            match callee {
                ir::IrCallee::Tensor => format!("Tensor::of({})", arg_code.join(", ")),
                ir::IrCallee::Linear => format!("nn::Linear({})", arg_code.join(", ")),
                ir::IrCallee::ReLU => format!("nn::ReLU({})", arg_code.join(", ")),
                ir::IrCallee::MatMul => format!("{} * {}", arg_code[0], arg_code[1]),
            }
        }
        ir::IrExpr::Unary { op: ir::UnaryOp::Neg, expr } => {
            format!("-({})", expr_to_cpp(expr)?)  
        }
        ir::IrExpr::Binary { left, right, op } => {
            let l = expr_to_cpp(left)?;
            let r = expr_to_cpp(right)?;
            match op {
                ir::BinaryOp::Add => format!("{} + {}", l, r),
                ir::BinaryOp::Mul => format!("{} * {}", l, r),
            }
        }
    })
}
