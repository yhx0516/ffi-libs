using System;
using System.Collections.Generic;
using System.IO;
using System.Numerics;
using System.Runtime.InteropServices;
using static System.Net.Mime.MediaTypeNames;
using static System.Net.WebRequestMethods;

namespace csharp_link_rust
{
    internal class Program
    {
        // ==================================================
        // HostServer Api
        // ==================================================
        [DllImport("http_service")]
        public static extern string http_service_get_version();

        // return HostServer ptr
        [DllImport("http_service")]
        public static extern IntPtr server_new(string ip, uint port);

        [DllImport("http_service")]
        public static extern void server_set_entry_dir(IntPtr ptr, string dir);

        [DllImport("http_service")]
        public static extern void server_set_threads(IntPtr ptr, uint threads);

        [DllImport("http_service")]
        public static extern void server_start(IntPtr ptr);

        [DllImport("http_service")]
        public static extern void server_close(IntPtr ptr);

        [DllImport("http_service")]
        public static extern void server_dispose(IntPtr ptr);

        [DllImport("http_service")]
        public static extern bool server_is_starting(IntPtr ptr);

        [DllImport("http_service")]
        public static extern bool server_is_running(IntPtr ptr);

        [DllImport("http_service")]
        public static extern bool server_is_closed(IntPtr ptr);

        [DllImport("http_service")]
        public static extern bool server_is_error(IntPtr ptr);

        [DllImport("http_service")]
        public static extern string server_get_error(IntPtr ptr);

        // ==================================================
        // Downloader Api
        // ==================================================
        // dl_api: like "http://xx/xx"
        // return Downloads ptr
        [DllImport("http_service")]
        public static extern IntPtr downloads_from_desc(string desc, string cache_dir, string dl_src);

        // toml_manifest_ptr: TomlManifest ptr, from manifest.toml
        // return Downloads ptr
        [DllImport("http_service")]
        public static extern IntPtr downloads_from_manifest_deps(
                IntPtr toml_manifest_ptr,
                string cache_dir,
                string addon_src,
                string addon_target
            );

        // ptr: Downloads ptr
        [DllImport("http_service")]
        public static extern void downloads_dispose(IntPtr ptr);


        // return Downloader ptr
        [DllImport("http_service")]
        public static extern IntPtr downloader_new();

        [DllImport("http_service")]
        public static extern void downloader_dispose(IntPtr ptr);

        [DllImport("http_service")]
        public static extern void downloader_set_threads(IntPtr ptr, uint threads);

        // dl_files_ptr: Downloads ptr
        [DllImport("http_service")]
        public static extern void downloader_set_downloads(IntPtr ptr, IntPtr downloads_ptr);

        [DllImport("http_service")]
        public static extern void downloader_start(IntPtr ptr);

        [DllImport("http_service")]
        public static extern bool downloader_is_finished(IntPtr ptr);

        [DllImport("http_service")]
        public static extern void downloader_cancel(IntPtr ptr);

        [DllImport("http_service")]
        public static extern uint downloader_get_total_count(IntPtr ptr);

        [DllImport("http_service")]
        public static extern uint downloader_get_downloaded_count(IntPtr ptr);

        [DllImport("http_service")]
        public static extern UInt64 downloader_get_total_size(IntPtr ptr);

        [DllImport("http_service")]
        public static extern UInt64 downloader_get_recv_size(IntPtr ptr);

        [DllImport("http_service")]
        public static extern bool downloader_is_download_valid(IntPtr ptr, uint idx);

        [DllImport("http_service")]
        public static extern string downloader_get_donwload_name(IntPtr ptr, uint idx);

        [DllImport("http_service")]
        public static extern string downloader_get_download_cache_path(IntPtr ptr, uint idx);

        [DllImport("http_service")]
        public static extern UInt64 downloader_get_download_total_size(IntPtr ptr, uint idx);

        [DllImport("http_service")]
        public static extern UInt64 downloader_get_download_recv_size(IntPtr ptr, uint idx);

        // DownloadStatus::Ready => 0x00,
        // DownloadStatus::Downloading => 0x01,
        // DownloadStatus::Downloaded => 0x02,
        // DownloadStatus::Unpacking => 0x03,
        // DownloadStatus::Done => 0x04,
        // DownloadStatus::Failed(_) => 0x05,
        [DllImport("http_service")]
        public static extern int downloader_get_download_status(IntPtr ptr, uint idx);

        [DllImport("http_service")]
        public static extern string try_get_err();

        // ==================================================
        // SimpleRequest Api
        // ==================================================
        // return SimpleRequest ptr
        [DllImport("http_service")]
        public static extern IntPtr simple_request_new(string url);

        [DllImport("http_service")]
        public static extern void simple_request_dispose(IntPtr ptr);

        [DllImport("http_service")]
        public static extern void simple_request_start(IntPtr ptr);

        [DllImport("http_service")]
        public static extern bool simple_request_is_finished(IntPtr ptr);

        [DllImport("http_service")]
        public static extern string simple_request_get_result(IntPtr ptr);

        // ==================================================
        // Addon Api
        // ==================================================
        [DllImport("addon")]
        public static extern string addon_get_version();

        // return TomlManifest ptr
        [DllImport("addon")]
        public static extern IntPtr toml_manifest_parse_file(string path);

        // return TomlManifest ptr
        [DllImport("addon")]
        public static extern IntPtr toml_manifest_parse_content(string content);

        [DllImport("addon")]
        public static extern void toml_manifest_dispose(IntPtr ptr);

        [DllImport("addon")]
        public static extern string toml_manifest_get_rt_name_with_version(IntPtr ptr);

        [DllImport("addon")]
        public static extern string toml_manifest_get_dep_version(IntPtr ptr, string key);

        [DllImport("addon")]
        public static extern string toml_manifest_get_dep_name_with_version(IntPtr ptr, string key);

        [DllImport("addon")]
        public static extern IntPtr toml_manifest_get_dep_names(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport("addon")]
        public static extern IntPtr toml_manifest_get_dep_names_with_version(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport("addon")]
        public static extern IntPtr toml_manifest_get_dep_paths(IntPtr ptr, string addon_path, string entry_path);

        // return Workspace ptr
        [DllImport("addon")]
        public static extern IntPtr workspace_new(string manifest_path);

        [DllImport("addon")]
        public static extern void workspace_dispose(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport("addon")]
        public static extern IntPtr workspace_get_members(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport("addon")]
        public static extern IntPtr workspace_get_member_names(IntPtr ptr);

        [DllImport("addon")]
        public static extern string workspace_find_member_by_name(IntPtr ptr, string name);

        // ==================================================
        // String Api
        // ==================================================
        [DllImport("addon")]
        public static extern uint strs_len(IntPtr ptr);

        [DllImport("addon")]
        public static extern string strs_get(IntPtr ptr, uint index);

        [DllImport("addon")]
        public static extern void strs_dispose(IntPtr ptr);

        [DllImport("addon")]
        public static extern void str_dispose(IntPtr ptr);

        static void Main(string[] args)
        {
            Console.WriteLine(http_service_get_version());

            // DownloadServerTest();
            // DownloaderTest();

            // AddonServerTest();
            // AddonTest();

            // WorkspaceTest();

            Console.ReadLine();
        }

        static void WorkspaceTest()
        {

            IntPtr ws_ptr = workspace_new("F:\\SoFunny\\froge\\tests\\addon\\manifest.toml");
            IntPtr members_ptr = workspace_get_members(ws_ptr);
            IntPtr member_names_ptr = workspace_get_member_names(ws_ptr);

            Console.WriteLine("members");
            foreach (string str in ConvertStrsPtr(members_ptr))
            {
                Console.WriteLine("  " + str);
            }
            Console.WriteLine();

            Console.WriteLine("member_names");
            foreach (string str in ConvertStrsPtr(member_names_ptr))
            {
                Console.WriteLine("  " + str);
            }
            Console.WriteLine();

            string addon_path = workspace_find_member_by_name(ws_ptr, "addon");
            Console.WriteLine("find \"addon\": " + addon_path);

            string foo_path = workspace_find_member_by_name(ws_ptr, "foo");
            Console.WriteLine("find \"foo\": " + foo_path);

            string bar_path = workspace_find_member_by_name(ws_ptr, "bar");
            Console.WriteLine("find \"bar\": " + bar_path);

            workspace_dispose(ws_ptr);
        }

        static void DownloadServerTest()
        {
            Console.WriteLine(http_service_get_version());

            IntPtr server_ptr = server_new("0.0.0.0", 7777);
            server_set_entry_dir(server_ptr, "F:\\SoFunny\\froge\\tests\\download");
            server_set_threads(server_ptr, 5);

            server_start(server_ptr);

            Console.ReadLine();
            server_close(server_ptr);
            server_dispose(server_ptr);
        }

        static void DownloaderTest()
        {
            string url = "http://127.0.0.1:7777";
            IntPtr req = simple_request_new(url);
            simple_request_start(req);
            while (!simple_request_is_finished(req)) { }
            string dl_desc = simple_request_get_result(req);
            simple_request_dispose(req);
            Console.WriteLine(dl_desc);

            string dl_src = url;
            string cache_dir = "./download";
            IntPtr downloads_ptr = downloads_from_desc(dl_desc, cache_dir, dl_src);

            IntPtr downloader_ptr = downloader_new();
            downloader_set_threads(downloader_ptr, 4);
            downloader_set_downloads(downloader_ptr, downloads_ptr);
            downloader_start(downloader_ptr);

            while (!downloader_is_finished(downloader_ptr))
            {
                UInt64 total_size = downloader_get_total_size(downloader_ptr);
                UInt64 recv_size = downloader_get_recv_size(downloader_ptr);
                uint total_files = downloader_get_total_count(downloader_ptr);
                uint completed_files = downloader_get_downloaded_count(downloader_ptr);
                Console.WriteLine("total_size: " + total_size
                        + ", recv_size: " + recv_size
                        + ", total_files: " + total_files
                        + ", completed_files: " + completed_files
                    );
            }

            Console.WriteLine("finished downloading");
            Console.ReadLine();
            // downloader_cancel(downloader_ptr);
            downloader_dispose(downloader_ptr);
            downloads_dispose(downloads_ptr);
        }

        static void AddonServerTest()
        {
            // 解析下载后的 manifest.toml
            string entry_dir = "F:/SoFunny/froge/tests/land";
            string manifest_path = entry_dir + "/manifest.toml";
            IntPtr manifest_ptr = toml_manifest_parse_file(manifest_path);

            // 获取 addon 下载文件
            string cache_dir = "./.cache";
            string addon_src = "https://dl.sofunny.io/addons-api/addons/";
            string addon_target = "./addons";
            IntPtr downloads_ptr = downloads_from_manifest_deps (
                    manifest_ptr,
                    cache_dir,
                    addon_src,
                    addon_target
                );

            string addon_path = "../addons";
            string entry_path = "../../.cache/land/127.0.0.1";
            IntPtr dep_paths = toml_manifest_get_dep_paths(manifest_ptr, addon_path, entry_path);
            foreach (string str in ConvertStrsPtr(dep_paths))
            {
                Console.WriteLine("  " + str);
            }


            // 下载 addons
            IntPtr addon_dl_ptr = downloader_new();
            downloader_set_threads(addon_dl_ptr, 4);
            downloader_set_downloads(addon_dl_ptr, downloads_ptr);
            downloader_start(addon_dl_ptr);

            while (!downloader_is_finished(addon_dl_ptr))
            {
                UInt64 total_size = downloader_get_total_size(addon_dl_ptr);
                UInt64 recv_size = downloader_get_recv_size(addon_dl_ptr);
                uint total_files = downloader_get_total_count(addon_dl_ptr);
                uint completed_files = downloader_get_downloaded_count(addon_dl_ptr);
                Console.WriteLine("total_size: " + total_size
                        + ", recv_size: " + recv_size
                        + ", total_files: " + total_files
                        + ", completed_files: " + completed_files
                    );
            }

            Console.WriteLine("downloaded " + downloader_get_downloaded_count(addon_dl_ptr) + " addon");
            downloader_dispose(addon_dl_ptr);
            downloads_dispose(downloads_ptr);

            // 启动资源服务器
            IntPtr server_ptr = server_new("0.0.0.0", 7778);
            server_set_entry_dir(server_ptr, entry_dir);
            server_set_threads(server_ptr, 4);
            server_start(server_ptr);

            Console.ReadLine();
            server_close(server_ptr);
            server_dispose(server_ptr);
        }

        static void AddonTest()
        {
            // 获取服务端局域网分享的下载列表
            string ip = "127.0.0.1";
            string dl_src = "http://127.0.0.1:7778";
            IntPtr dl_req = simple_request_new(dl_src);
            simple_request_start(dl_req);
            while (!simple_request_is_finished(dl_req)) { }

            string dl_desc = simple_request_get_result(dl_req);
            simple_request_dispose(dl_req);
            Console.WriteLine(dl_desc);
            Console.WriteLine("");

            // 下载局域网分享的文件
            string land_cache_dir = "./.cache/lands/" + ip;
            IntPtr downloads_ptr = downloads_from_desc(dl_desc, land_cache_dir, dl_src);
            IntPtr downloader_ptr = downloader_new();
            downloader_set_threads(downloader_ptr, 4);
            downloader_set_downloads(downloader_ptr, downloads_ptr);
            downloader_start(downloader_ptr);

            while (!downloader_is_finished(downloader_ptr))
            {
                UInt64 total_size = downloader_get_total_size(downloader_ptr);
                UInt64 recv_size = downloader_get_recv_size(downloader_ptr);
                uint total_files = downloader_get_total_count(downloader_ptr);
                uint completed_files = downloader_get_downloaded_count(downloader_ptr);
                Console.WriteLine("total_size: " + total_size
                        + ", recv_size: " + recv_size
                        + ", total_files: " + total_files
                        + ", completed_files: " + completed_files
                    );
            }

            Console.WriteLine("downloaded " + downloader_get_downloaded_count(downloader_ptr) + " sharing files");
            downloader_dispose(downloader_ptr);
            downloads_dispose(downloads_ptr);

            // 解析下载后的 manifest.toml
            string manifest_path = land_cache_dir + "/manifest.toml";
            IntPtr manifest_ptr = toml_manifest_parse_file(manifest_path);

            // 获取 addon 下载文件
            string cache_dir = "./.cache";
            string addon_src = "https://dl.sofunny.io/addons-api/addons/";
            string addon_target = "./addons";
            IntPtr addons_dl_files_prt = downloads_from_manifest_deps(
                    manifest_ptr,
                    cache_dir,
                    addon_src,
                    addon_target
                );

            // 下载 addons
            IntPtr addon_dl_ptr = downloader_new();
            downloader_set_threads(addon_dl_ptr, 4);
            downloader_set_downloads(addon_dl_ptr, addons_dl_files_prt);
            downloader_start(addon_dl_ptr);

            while (!downloader_is_finished(addon_dl_ptr))
            {
                UInt64 total_size = downloader_get_total_size(addon_dl_ptr);
                UInt64 recv_size = downloader_get_recv_size(addon_dl_ptr);
                uint total_files = downloader_get_total_count(addon_dl_ptr);
                uint completed_files = downloader_get_downloaded_count(addon_dl_ptr);
                Console.WriteLine("total_size: " + total_size
                        + ", recv_size: " + recv_size
                        + ", total_files: " + total_files
                        + ", completed_files: " + completed_files
                    );
            }

            Console.WriteLine("downloaded " + downloader_get_downloaded_count(addon_dl_ptr) + " addon");
            downloader_dispose(addon_dl_ptr);
            downloads_dispose(addons_dl_files_prt);
            // downloader_cancel(downloader_ptr);

            Console.ReadLine();
        }

        private static string[] ConvertStrsPtr(System.IntPtr strs_ptr)
        {
            if (strs_ptr != System.IntPtr.Zero)
            {
                UInt32 len = strs_len(strs_ptr);
                if (len != 0)
                {
                    string[] files = new string[len];
                    for (uint i = 0; i < len; i++)
                    {
                        files[i] = strs_get(strs_ptr, i);
                    }

                    strs_dispose(strs_ptr);
                    return files;
                }
                strs_dispose(strs_ptr);
            }
            return new string[0];
        }
    }
}
