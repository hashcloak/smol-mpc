use smol::utils::prg::Prg;

#[test]
fn create_prg_default() {
    let mut prg = Prg::new(None);
    let random_stream = prg.next(2);

    let mut prg2 = Prg::new(Some(vec![0; 32]));
    let random_stream2 = prg2.next(2);

    assert_eq!(random_stream, random_stream2);
}

#[test]
fn create_prg_autocomplete() {
    let seed = vec![0x24; 30];
    let real_seed = vec![
        0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24,
        0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24,
        0, 0,
    ];

    let mut prg = Prg::new(Some(seed));
    let mut prg_real = Prg::new(Some(real_seed));

    let random_stream = prg.next(2);
    let random_stream_real = prg_real.next(2);

    assert_eq!(random_stream, random_stream_real);
}
