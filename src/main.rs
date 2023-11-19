use math_sheets::*;

fn main() {
    println!(
        r#"<html>
    <head><title>Math Sheet</title>
    <style>
    body {{
        align: center;
    }}
    .equation {{
        pad: 10px;
        margin: 10px;
        align: center;
    }}
    .operand {{
        margin: 5px;
    }}
    .operator {{
        margin: 1px;
    }}
    .answer {{
        width: 50px;
        height: 50px;
        padding: 1px;
        marging: 5px;
        border: thin solid black;
    }}
    </style>
    </head>
    <body>"#
    );
    let addition_list = create_addition_list(20, 3, 18);
    for addition_equation in addition_list {
        println!(
            r#"<div class="equation"><span class="operand">{}</span><span class="operator">{}</span><span class="operand">{}</span><span class="equal-sign">=</span><span class="answer">&emsp;&emsp;</span></div>"#,
            addition_equation.first_operand,
            addition_equation.operator,
            addition_equation.second_operand,
        );
    }
    println!("</body></html>")
}
