use math_sheets::*;

fn main() {
    println!(
        r#"<html>
    <head><title>Math Sheet</title>
    <style>
    body {{
        align: center;
    }}
    .equations {{
        display: grid;
        gap: 10px;
    }}
    .equation {{
        display: flex;
    }}
    .operand-one {{
        margin: 1px;
    }}
    .operand-two {{
        margin: 1px;
    }}
    .operator {{
        margin: 1px;
    }}
    .answer {{
        border: thin solid black;
    }}
    </style>
    </head>
    <body>
    <div class="equations">"#
    );
    let addition_list = create_addition_list(20, 3, 18);
    for addition_equation in addition_list {
        println!(
            r#"<div class="equation"><span class="operand-one">{}</span><span class="operator">{}</span><span class="operand-two">{}</span><span class="equal-sign">=</span><span class="answer">&emsp;&emsp;</span></div>"#,
            addition_equation.first_operand,
            addition_equation.operator,
            addition_equation.second_operand,
        );
    }
    println!("</div></body></html>")
}
