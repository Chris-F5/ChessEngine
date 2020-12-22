use crate::PieceType;
use ggez::{graphics::Image, Context};

pub struct PieceSetImages {
    pawn: Image,
    bishop: Image,
    knight: Image,
    rook: Image,
    queen: Image,
    king: Image,
}

impl PieceSetImages {
    pub fn get_image(&self, piece_type: PieceType) -> &Image {
        match piece_type {
            PieceType::Pawn => &self.pawn,
            PieceType::Bishop => &self.bishop,
            PieceType::Knight => &self.knight,
            PieceType::Rook => &self.rook,
            PieceType::Queen => &self.queen,
            PieceType::King => &self.king,
        }
    }
}

pub fn load_board_image(ctx: &mut Context) -> Image {
    Image::new(ctx, "/board.png").unwrap()
}

pub fn load_white_piece_set(ctx: &mut Context) -> PieceSetImages {
    PieceSetImages {
        pawn: Image::new(ctx, "/white_pawn.png").unwrap(),
        bishop: Image::new(ctx, "/white_bishop.png").unwrap(),
        knight: Image::new(ctx, "/white_knight.png").unwrap(),
        rook: Image::new(ctx, "/white_rook.png").unwrap(),
        queen: Image::new(ctx, "/white_queen.png").unwrap(),
        king: Image::new(ctx, "/white_king.png").unwrap(),
    }
}

pub fn load_black_piece_set(ctx: &mut Context) -> PieceSetImages {
    PieceSetImages {
        pawn: Image::new(ctx, "/black_pawn.png").unwrap(),
        bishop: Image::new(ctx, "/black_bishop.png").unwrap(),
        knight: Image::new(ctx, "/black_knight.png").unwrap(),
        rook: Image::new(ctx, "/black_rook.png").unwrap(),
        queen: Image::new(ctx, "/black_queen.png").unwrap(),
        king: Image::new(ctx, "/black_king.png").unwrap(),
    }
}
