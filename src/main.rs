extern crate nifpga;

use std::path::PathBuf;
use std::process;
use clap::{Parser};
use nifpga::{NifpgaError, Session, ReadFifo, WriteFifo, ReadElements};
use std::thread;
use std::sync::mpsc;


#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Cli {
    /// Set the bit_file name
    bit_file: Option<PathBuf>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "SIGNATURE",  default_value = "")]
    signature: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, value_name = "NI_ADDR",  default_value = "RIO0")]
    ni_address: Option<String>,

    #[arg(short, long, value_name = "false",  default_value = "false")]
    run: Option<bool>,

    #[arg(short,long, value_name = "false",  default_value = "false")]
    close_on_reset: Option<bool>,

    #[arg(short, long, value_name = "0",  default_value = "0")]
    fifo: Option<u32>,

    #[arg(long, value_name = "50000",  default_value = "50000")]
    dma_buffer_size: Option<usize>,

    #[arg(long, value_name = "5000",  default_value = "5000")]
    fifo_reading_buffer: Option<usize>,

    #[arg(short, long, value_name = "13123",  default_value = "13123")]
    port: Option<usize>,
}

struct Configuration {
    bit_file: String,
    signature: String,
    ni_address: String,
    run: bool,
    close_on_reset: bool,
    fifo: u32,
    port: usize,
    dma_buffer_size: usize,
    fifo_reading_buffer: usize,
}

impl Configuration{
    fn new()->Configuration{
        Configuration {
            bit_file: String::from(""),
            signature: String::from(""),
            ni_address: String::from(""),
            run: false,
            close_on_reset: false,
            fifo: 0,
            port: 0,
            dma_buffer_size: 0,
            fifo_reading_buffer: 0,
        }
    }
}

fn import_args_as_configuration() -> Configuration{
    let cli = Cli::parse();
    let mut conf: Configuration = Configuration::new();

    if let Some(bit_file) = cli.bit_file {
        conf.bit_file = bit_file.display().to_string();
        println!("bit file: {}", conf.bit_file);
    } else {
        println!("THe BIT FILE IS NEEDED!");
        process::exit(0x0);
    }

    if let Some(ni_address) = cli.ni_address {
        conf.ni_address = ni_address;
        println!("ni_address: {}", conf.ni_address);
    }

    if let Some(signature) = cli.signature {
        conf.signature = signature;
        println!("signature: {}", conf.signature);
    }

    if let Some(run) = cli.run {
        conf.run = run;
        println!("Run: {}", conf.run);
    }

    if let Some(close_on_reset) = cli.close_on_reset {
        conf.close_on_reset = close_on_reset;
        println!("close_on_reset: {}", conf.close_on_reset);
    }


    if let Some(fifo) = cli.fifo {
        conf.fifo = fifo;
        println!("fifo: {}", conf.fifo);
    }

    if let Some(port) = cli.port {
        conf.port = port;
        println!("port: {}", conf.port);
    }

    if let Some(dma_buffer_size) = cli.dma_buffer_size {
        conf.dma_buffer_size = dma_buffer_size;
        println!("dma_buffer_size: {}", conf.dma_buffer_size);
    }

    if let Some(fifo_reading_buffer) = cli.fifo_reading_buffer {
        conf.fifo_reading_buffer = fifo_reading_buffer;
        println!("fifo_reading_buffer: {}", conf.fifo_reading_buffer);
    }

    conf
}


// thread::spawn(move || {
//
// });


fn main() -> Result<(), NifpgaError>{
   let conf = import_args_as_configuration();

    let session = Session::open(
        conf.bit_file.as_str(),
        conf.signature.as_str(),//signature from generated header
        conf.ni_address.as_str(),
        conf.run, //run on open
        conf.close_on_reset //close_on_reset on close
    )?;

    let (reader, depth) = session.open_read_fifo::<u64>(conf.fifo, conf.dma_buffer_size)?;

    println!("Actual FIFO {} set depth: {} actual depth: {}", conf.fifo, conf.dma_buffer_size, depth);
    println!("conf.fifo_reading_buffer: {}", conf.fifo_reading_buffer);


    //read_buff.resize(conf.fifo_reading_buffer, 0);
   // let (tx, rx) = mpsc::channel();

        let mut read_buff:Vec<u64> = Vec::with_capacity(conf.fifo_reading_buffer);
        read_buff.resize(conf.fifo_reading_buffer, 0);

        let mut read_buff_zerosized:Vec<u64> = Vec::with_capacity(0);

        loop {
            let data_available = reader.read(&mut read_buff_zerosized, 0)?;
            if (data_available>0) {
                read_buff.resize(data_available, 0);
                let r = reader.read(&mut read_buff, 0)?;
                println!("r {:?} {:?}", data_available, r);
               // tx.send(read_buff.to_vec());
            }
        }




    loop {
        println!("Received {:?}", rx.recv());
    }





    Ok(())
}



