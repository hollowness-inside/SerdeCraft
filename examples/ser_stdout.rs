use std::io::{BufWriter, Write};

use serde::Serialize;
use serdecraft::MCWebSocket;

struct StdoutMCSocket<W> {
    out: W,
    buffer: Vec<serdecraft::MinecraftBlock>,
}

impl<W: Write> MCWebSocket for StdoutMCSocket<W> {
    fn send_block(&mut self, block: serdecraft::MinecraftBlock) -> serdecraft::MinecraftResult<()> {
        if self.buffer.len() >= 5 {
            self.flush()?;
        }
        self.buffer.push(block);
        Ok(())
    }

    fn consume_block(&mut self) -> serdecraft::MinecraftResult<serdecraft::MinecraftBlock> {
        todo!()
    }

    fn skip_block(&mut self) -> serdecraft::MinecraftResult<()> {
        todo!()
    }

    fn rewind_block(&mut self) -> serdecraft::MinecraftResult<()> {
        todo!()
    }

    fn flush(&mut self) -> serdecraft::MinecraftResult<()> {
        println!("Flushing {} blocks", self.buffer.len());
        let s = self
            .buffer
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        self.out.write_all(s.as_bytes())?;
        self.buffer.clear();
        Ok(())
    }
}

#[derive(serde::Serialize)]
struct Example {
    a: i32,
    b: String,
    c: Vec<f64>,
}

fn main() {
    let writer = vec![];
    let writer = BufWriter::new(writer);
    let w = StdoutMCSocket {
        out: writer,
        buffer: Vec::new(),
    };
    let mut serializer = serdecraft::MinecraftSerializer::new(w);

    let example = Example {
        a: 42,
        b: "Hello, Minecraft!".to_string(),
        c: vec![1.0, 2.0, 3.0],
    };

    example.serialize(&mut serializer).unwrap();
}
