use std::mem::zeroed;
use std::process::exit;
use std::ptr::{null_mut};
use widestring::U16String;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::minwinbase::{ WIN32_FIND_DATAW};
use winapi::um::wininet::{FTP_TRANSFER_TYPE_BINARY, FtpFindFirstFileW, FtpPutFileW, FtpSetCurrentDirectoryW, HINTERNET, INTERNET_FLAG_PASSIVE, INTERNET_SERVICE_FTP, InternetConnectW, InternetFindNextFileW, InternetOpenW};


fn main() {
    unsafe{
    let mut ftp_internet_session = zeroed::<HINTERNET>();
    let mut ftp_connect = zeroed::<HINTERNET>();

        let user_agent = format!("Mozilla/5.0 (Windows NT 10.0; Win64; x64\0").as_ptr();
        let mut host = U16String::from_str("somehost.com\0");
        let mut user = U16String::from_str("someuser\0");
        let mut password = U16String::from_str("somepassword\0");
        let  port:u16 =21;


        ftp_internet_session = InternetOpenW(
            user_agent as *const u16,
            1,
            null_mut(),
            null_mut(),
            0,
        );

        if ftp_internet_session.is_null(){
            println!("Internet OpenW Fail ");
            exit(1);
        }
        println!("IntertOpenW Success");

        ftp_connect = InternetConnectW(
            ftp_internet_session,
            host.as_ptr(),
            port,
            user.as_ptr(),
            password.as_ptr(),
            INTERNET_SERVICE_FTP,
            INTERNET_FLAG_PASSIVE,
            1,
        );

        if ftp_connect.is_null(){
            println!("InternetConnectW fail {}",GetLastError());
            exit(2);
        }
        println!("InternetConnectW Success");
        //TODO HERE START TO LIST THE FILES AND MOVE TO DIR
        let mut file_name = zeroed::<WIN32_FIND_DATAW>();
        let mut ftp_file = FtpFindFirstFileW(
            ftp_connect,
            null_mut(),
            &mut file_name as *mut _,
            0x80000000,
            0,

        );

        if ftp_file.is_null(){
            println!("Fail FtpFindFirstFileW");
            exit(3);
        }

        println!("Success FtpFindFirstFileW");


        loop{
            let mut file_name_2 = zeroed::<WIN32_FIND_DATAW>();
           let status =  InternetFindNextFileW(
               ftp_file,
               &mut file_name_2 as *const _ as *mut _,
            );

            if status == 0{
                println!("Fail InternetFindNextFileW");
                break;
            }

            //println!("Success InrnetFindNextFileW");

            //TODO NOW WE CLEAN THE ARRAY

            let clean_name_file = file_name_2.cFileName.into_iter().take_while(|x|*x  != 0).collect::<Vec<u16>>();
            let final_name = String::from_utf16(clean_name_file.as_slice()).unwrap();

            println!("Name File {}",final_name);
            //TODO here change directory and upload the file
            match final_name.as_str(){
                "domains"=>{
                    let format_domains =  U16String::from_str(final_name.as_str());
                    let change_to_domains =FtpSetCurrentDirectoryW(
                        ftp_connect,
                        format_domains.as_ptr(),
                    );

                    if change_to_domains == 0{
                        println!("fail change domains folder");
                        exit(5);
                    }
                    println!("Success Change Domains Folder");



                    let change_to_domain_folder =FtpSetCurrentDirectoryW(
                        ftp_connect,
                        host.as_ptr(),
                    );
                    if change_to_domain_folder == 0{
                        println!("fail change Domain folder");
                        exit(5);
                    }
                    println!("Success Change Domain Folder");

                    let format_public_html =U16String::from_str("public_html");
                    let change_to_public_html_folder =FtpSetCurrentDirectoryW(
                        ftp_connect,
                        format_public_html.as_ptr(),
                    );
                    if change_to_public_html_folder == 0{
                        println!("fail change public_html folder");
                        exit(5);
                    }
                    println!("Success Change public_html Folder");

                    let name_file_local = U16String::from_str("SomeFile\0");
                   let put_file =  FtpPutFileW(
                       ftp_connect,
                       name_file_local.as_ptr(),
                       name_file_local.as_ptr(),
                       FTP_TRANSFER_TYPE_BINARY,
                       0,

                   );

                    if put_file == 0 {
                        println!("Fail upload file {}",GetLastError());
                        exit(5);
                    }
                    println!("Succes Upload");
                    break;
                },
                "public_html"=>{
                    let format_public_html =U16String::from_str(final_name.as_str());
                    let change_to_public_html_folder =FtpSetCurrentDirectoryW(
                        ftp_connect,
                        format_public_html.as_ptr(),
                    );
                    if change_to_public_html_folder == 0{
                        println!("fail change public_html folder");
                        exit(5);
                    }
                    println!("Success Change public_html Folder");

                    let name_file_local = U16String::from_str("SomeFile\0");
                    let put_file =  FtpPutFileW(
                        ftp_connect,
                        name_file_local.as_ptr(),
                        name_file_local.as_ptr(),
                        FTP_TRANSFER_TYPE_BINARY,
                        0,

                    );

                    if put_file == 0 {
                        println!("Fail upload file {}",GetLastError());
                        exit(5);
                    }
                    println!("Succes Upload");
                    break;
                },
                _=>{
                    println!("No actions with this file");
                }
            }


        }

    }
}
