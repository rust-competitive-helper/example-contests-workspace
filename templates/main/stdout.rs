    let mut stdout = std::io::stdout();
    let output = if $INTERACTIVE {
        crate::algo_lib::io::output::Output::new_with_auto_flush(&mut stdout)
    } else {
        crate::algo_lib::io::output::Output::new(&mut stdout)
    };
