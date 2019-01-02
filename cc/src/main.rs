#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

extern crate regex;

mod fileutil;

use std::fs::*;
use std::io::*;
use std::env::*;
use std::collections::*;
use regex::*;
use crate::fileutil::*;


#[derive(Debug)]
struct Data
{
    input : Option<String>,
    output : Option<String>,
    include_dir : Option<String>,
}

#[derive(Debug)]
enum ParseState
{
    SetOutput,
    SetInput,
    SetInclude,
}

fn Parse(mut args : VecDeque<String>, mut data : &mut Data, state : ParseState)
{
    if args.len() == 0 { return; }
    let token = args.pop_front().unwrap();
    match state
    {
        ParseState::SetInput => match token.as_str()
        {
            "-i" => Parse(args, &mut data, ParseState::SetInclude),
            "-o" => Parse(args, &mut data, ParseState::SetOutput),
            _ =>
            {
                if data.input != None { panic!("\ninput file re-defined!\n"); }
                data.input = Some(token);
                Parse(args, &mut data, ParseState::SetInput);
            }
        }
        
        ParseState::SetOutput =>
        {
            if data.output != None { panic!("\noutput file re-defined!\n"); }
            data.output = Some(token);
            Parse(args, &mut data, ParseState::SetInput);
        }
        
        ParseState::SetInclude =>
        {
            if data.include_dir != None
            { 
                panic!("\ninclude directory re-defined! \n".to_owned() +
                    "code expension only allowed in a single directory, \n" +
                    "while you can use relative path in #include command.\n"
                    );
            }
            data.include_dir = Some(token);
            Parse(args, &mut data, ParseState::SetInput);
        }
    }
}

fn main()
{
    
    let mut data = Data {
        input : None,
        output : None,
        include_dir : None,
    };
    
    let mut args : VecDeque<String> = args().map(|x| x.to_string()).collect();
    args.pop_front().unwrap();
    Parse(args, &mut data, ParseState::SetInput);
    
    if data.output == None { data.output = Some("submit.cpp".to_owned()); }
    if data.input == None { panic!("You should specify an input file!"); }
    if data.include_dir == None { data.include_dir = Dir((&data).input.as_ref().unwrap()); }
    if data.include_dir == None { panic!("You must specify an include path while directory of input file is invalid."); }
    
    let source = { ReadAll((&data).input.as_ref().unwrap()) };
    if source == None { panic!("Can not read from file {}", &data.input.unwrap()); }
    
    // println!("{:?}", data);
    
    let input = data.input.unwrap();
    let output = data.output.unwrap();
    let include = data.include_dir.unwrap();
    let re = Regex::new("#include[ ]*\"(.*)\"").unwrap();
    let sourceLines = source.unwrap().split('\n').map(|x| x.trim_right().to_string()).collect::<Vec<String>>();
    let mut dstLines = Vec::new();
    let mut transfering = false;
    for line in sourceLines
    {
        match line.as_str()
        {
            "/* bruteforces generation begin */" =>
            {
                transfering = true;
                dstLines.push("/// >>> bruteforces generated begin.".to_string());
            }
            
            "/* bruteforces generation end */" =>
            {
                transfering = false;
                dstLines.push("/// >>> bruteforces generated end.".to_string());
            }
            
            _ =>
            {
                if !transfering
                {
                    dstLines.push(line);
                    continue;
                }
                
                let mut capped = false;
                for cap in re.captures_iter(&line)
                {
                    capped = true;
                    let incfile = include.to_owned() + "/" + &cap[1];
                    match ReadAll(&incfile)
                    {
                        Some(ref s) => { dstLines.append(&mut s.split('\n').map(|x| x.to_string()).collect::<Vec<String>>()); }
                        None => { panic!("cannot transfer included file {}", incfile); }
                    }
                    break; // only take the first capture.
                }
                if !capped { dstLines.push(line); }
            }
        }
    }
    
    WriteAll(&output, &dstLines.join("\n"));
}
