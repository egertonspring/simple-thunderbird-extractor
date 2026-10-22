# simple-thunderbird-extractor

## Description
This tool is written in Rust without any dependencies and allows to split
an MBOX file into single EML files which can be opened with a mail programme
as well as a text editor. You can then simply view emails as single files
and also extract their attachments. This tool is complete as it is. You are
welcome to fork it and add your own features.

## Usage
```
cargo build --release
./target/release/simple-thunderbird-extractor EMAIL.mbox
```

After that your can find EML files in the pwd. Sometimes thousands of them.

## Out of scope
This tool is a very simple text splitting tool. This is not written for Outlook.pst
of Outlook.ost files.
