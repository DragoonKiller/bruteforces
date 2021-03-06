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
    hint : Option<bool>,
    remove_comment : Option<bool>,
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
            "-u" => data.hint = Some(false),
            "-m" => data.remove_comment = Some(false),
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

fn Unfold(
    filename : &str,            // current open file.
    incdir : &str,              // include directory.
    mut predef : &mut Vec<String>,  // already unfolded file names.
    mut info : &mut Vec<String>,    // currently unfolding file names.
    hint : bool,                // whether output hints.
    ingore_comment : bool,      // whether ignore comments.
) -> Vec<String>
{
    let srcfile = NormPath(filename);
    if(srcfile == None) { panic!("source file path [{}] invalid!", filename); }
    
    info.push(srcfile.unwrap().to_owned());
    if info.len() > 32 { panic!("recursion too deep! with \n{}", info.join("\n")); }
    
    let mut source = ReadAll(filename);
    if source == None { panic!("Can not read from file [{}] from \n{}", filename, info.join("\n")); }
    
    if ingore_comment // will **not** trim comments **generated** inside.
    {
        let re_linecomm = Regex::new("//.*\n").unwrap();
        let re_blockcomm = Regex::new("/\\*.*\\*/").unwrap();
        source = Some(re_blockcomm.replace_all(&source.unwrap(), "").to_string());
        source = Some(re_linecomm.replace_all(&source.unwrap(), "").to_string());
    }
    
    let re_inc : Regex = Regex::new("^[ ]*#bruteforces[ ]*include[ ]*\"(.*)\"").unwrap();
    let src = source.unwrap().split('\n').map(|x| x.trim_right().to_string()).collect::<Vec<String>>();
    let mut dst = Vec::new();
    'nextline: for line in src
    {
        for cap in re_inc.captures_iter(&line)
        {
            let incpath = NormPath(incdir);
            if incpath == None { panic!("include directroy [{}] invalid!", incdir); }
            let incfile = incpath.unwrap() + "/" + &cap[1];
            let mut unfolded = Unfold(&incfile, incdir, &mut predef, &mut info, hint, ingore_comment);
            if predef.contains(&incfile.to_owned()) { break; }
            if hint { dst.push("// bruteforces >>> ".to_owned() + &incfile + " >>>"); }
            dst.append(&mut unfolded);
            if hint { dst.push("// <<< ".to_owned() + &incfile + " <<< bruteforces"); }
            predef.push(incfile);
            continue 'nextline; // only take the first capture of this line, if any.
        }
        
        dst.push(line);
    }
    info.pop();
    dst
}

fn main()
{
    
    let mut data = Data {
        input : None,
        output : None,
        include_dir : None,
        hint : None,
        remove_comment : None,
    };
    
    let mut args : VecDeque<String> = args().map(|x| x.to_string()).collect();
    args.pop_front().unwrap();
    Parse(args, &mut data, ParseState::SetInput);
    
    if data.output == None { data.output = Some("submit.cpp".to_owned()); }
    if data.input == None { panic!("You should specify an input file!"); }
    if data.include_dir == None { data.include_dir = Dir((&data).input.as_ref().unwrap()); }
    if data.include_dir == None { panic!("You must specify an include path while directory of input file is invalid."); }
    if data.hint == None { data.hint = Some(true); }
    if data.remove_comment == None { data.remove_comment = Some(true); }
    
    let dst = Unfold(
        (&data).input.as_ref().unwrap(),
        (&data).include_dir.as_ref().unwrap(),
        &mut Vec::new(),
        &mut Vec::new(),
        data.hint.unwrap(),
        data.remove_comment.unwrap(),
    ).join("\n");
    
    let output = data.output.unwrap();
    WriteAll(&output, &dst);
}
