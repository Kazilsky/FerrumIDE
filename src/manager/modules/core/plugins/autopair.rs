fn insert_pair(ctx: &mut Context, open: char, close: char) {
    ctx.buffer.enter_char(ctx.cursor.character_index, open);
    let len = ctx.buffer.strokes_array[ctx.buffer.present_stroke].len();
    ctx.cursor.move_right(len);

    // курсор теперь после '(', закрывающая вставляется на его место
    // и курсор остаётся между скобками
    ctx.buffer.enter_char(ctx.cursor.character_index, close);
}

pub fn autopair() -> Plugin {
    Plugin::new("autopair")
        .command("pair.paren",   |ctx| insert_pair(ctx, '(', ')'))
        .command("pair.bracket", |ctx| insert_pair(ctx, '[', ']'))
        .command("pair.brace",   |ctx| insert_pair(ctx, '{', '}'))
        .bind(KeyCode::Char('('), "pair.paren")
        .bind(KeyCode::Char('['), "pair.bracket")
        .bind(KeyCode::Char('{'), "pair.brace")
}

// при старте
pm.load(autopair());