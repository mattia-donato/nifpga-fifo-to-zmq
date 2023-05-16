extern crate nifpga;

use std::path::PathBuf;
use std::process;
use clap::{Parser};
use nifpga::{NifpgaError, Session, ReadFifo, WriteFifo};


//     "/home/admin/fpga.lvbitx",
//     "0DA1668CDC2C6C492F1437AE6DC2151E",//signature from generated header
//     "RIO0",
//     true, //run on open
//     true //close_on_reset on close

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Cli {
    /// Set the bitfile name
    bitfile: Option<PathBuf>,

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
    fifo: Option<usize>,

    #[arg(short, long, value_name = "13123",  default_value = "13123")]
    port: Option<usize>,
}

struct Configuration {
    bitfile: String,
    signature: String,
    ni_address: String,
    run: bool,
    close_on_reset: bool,
    fifo: usize,
    port: usize,
}

fn main() -> Result<(), NifpgaError>{
    let cli = Cli::parse();
    //
    let mut conf: Configuration = Configuration {
        bitfile: String::from(""),
        signature: String::from(""),
        ni_address: String::from(""),
        run: false,
        close_on_reset: false,
        fifo: 0,
        port: 13123,
    };
    // conf.
    //
    // bitfile: Option<PathBuf>
    // signature: Option<String>,
    // ni_address: Option<String>,
    // run: Option<bool>,
    // close_on_reset: Option<bool>,
    // fifo: Option<usize>,
    // port: Option<usize>,

    if let Some(bitfile) = cli.bitfile {
        conf.bitfile = bitfile.display().to_string();
        println!("bit file: {}", conf.bitfile);
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


    let session = Session::open(
        conf.bitfile.as_str(),
        conf.signature.as_str(),//signature from generated header
        conf.ni_address.as_str(),
        conf.run, //run on open
        conf.close_on_reset //close_on_reset on close
    )?;

    Ok(())

}


