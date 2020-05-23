#[derive(Clone, Debug)]
pub struct GPU {
    //cells: Vec<Cell>,
    cells: Vec<u8>,
    width: usize,
    height: usize,
    // Should always be the same size as `cells`. When updating, we read from
    // `cells` and write to `scratch_cells`, then swap. Otherwise it's not in
    // use, and `cells` should be updated directly.
    //scratch_cells: Vec<Cell>,
}

impl GPU {
    // ctor returning a self pointer
    pub fn new(width: usize, height: usize) -> Self {
        let size = width.checked_mul(height).expect("too big");
        Self {
            cells: vec![0; size],
            width,
            height,
        }
    }

    pub fn draw(&self, screen: &mut [u8]) {
        //debug_assert_eq!(screen.len(), 4 * self.cells.len());
        for (cell, pixels) in self.cells.iter().zip(screen.chunks_exact_mut(4)) {
            let color = if cell > &0 {
                [0x00, 0xff, 0x00, 0xff]
            } else {
                [0x00, 0x00, 0xff, 0xff]
            };
            pixels.copy_from_slice(&color);
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize) {
        self.cells[0] = 1;

        self.cells[self.width * y + x] = 1;
    }

    pub fn set_line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize) {
        // probably should do sutherland-hodgeman if this were more serious.
        // instead just clamp the start pos, and draw until moving towards the
        // end pos takes us out of bounds.
        let x0 = x0.max(0).min(self.width as isize);
        let y0 = y0.max(0).min(self.height as isize);
        for (x, y) in line_drawing::Bresenham::new((x0, y0), (x1, y1)) {
            self.set_pixel(x as usize, y as usize);
        }
    }
}
