use anyhow::Result;
use chess_fen_parser::{FenParser, Rule, parse_fen};
use pest::Parser;

#[test]
fn piece_rule() -> Result<()> {
    assert!(FenParser::parse(Rule::piece, "p").is_ok());
    assert!(FenParser::parse(Rule::piece, "K").is_ok());
    Ok(())
}

#[test]
fn rank_rule() -> Result<()> {
    assert!(FenParser::parse(Rule::rank, "rnbqkbnr").is_ok());
    assert!(FenParser::parse(Rule::rank, "8").is_ok());
    assert!(FenParser::parse(Rule::rank, "r3k2r").is_ok());
    Ok(())
}

#[test]
fn piece_placement_rule() -> Result<()> {
    let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    assert!(FenParser::parse(Rule::piece_placement, s).is_ok());
    Ok(())
}

#[test]
fn active_color_rule() -> Result<()> {
    assert!(FenParser::parse(Rule::active_color, "w").is_ok());
    assert!(FenParser::parse(Rule::active_color, "b").is_ok());
    Ok(())
}

#[test]
fn castling_rule() -> Result<()> {
    assert!(FenParser::parse(Rule::castling, "-").is_ok());
    assert!(FenParser::parse(Rule::castling, "KQkq").is_ok());
    Ok(())
}

#[test]
fn en_passant_and_square_rules() -> Result<()> {
    assert!(FenParser::parse(Rule::en_passant, "-").is_ok());
    assert!(FenParser::parse(Rule::en_passant, "e3").is_ok());
    assert!(FenParser::parse(Rule::square, "a1").is_ok());
    Ok(())
}

#[test]
fn rank_digit_rule() -> Result<()> {
    assert!(FenParser::parse(Rule::rank_digit, "8").is_ok());
    Ok(())
}

#[test]
fn halfmove_and_fullmove_rules() -> Result<()> {
    assert!(FenParser::parse(Rule::halfmove, "0").is_ok());
    assert!(FenParser::parse(Rule::fullmove, "1").is_ok());
    Ok(())
}

#[test]
fn full_fen_parse() -> Result<()> {
    let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let fen = parse_fen(s)?;
    assert_eq!(fen.board.len(), 8);
    assert_eq!(fen.active_color as u8, 0);
    assert!(fen.castling.white_short);
    assert!(fen.castling.white_long);
    assert!(fen.castling.black_short);
    assert!(fen.castling.black_long);
    assert!(fen.en_passant.is_none());
    assert_eq!(fen.halfmove_clock, 0);
    assert_eq!(fen.fullmove_number, 1);
    Ok(())
}

#[test]
fn fen_with_en_passant() -> Result<()> {
    let s = "8/8/8/3pP3/8/8/8/8 b - e3 0 1";
    let fen = parse_fen(s)?;
    assert_eq!(fen.en_passant.as_deref(), Some("e3"));
    Ok(())
}
