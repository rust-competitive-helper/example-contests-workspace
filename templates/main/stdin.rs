    let mut sin = std::io::stdin();
    let input = if $INTERACTIVE {
        crate::algo_lib::io::input::Input::new_with_size(&mut sin, 1)
    } else {
        crate::algo_lib::io::input::Input::new(&mut sin)
    };
