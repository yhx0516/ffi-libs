using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.InteropServices;
using System.Text;

namespace csharp_link_rust.libs
{
    public class rpkg
    {
        // ===============================================
        // Info
        // ===============================================
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern string get_version();

        // ===============================================
        // scan api
        // ===============================================
        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr rpkg_scan_files(
                [MarshalAs(UnmanagedType.LPUTF8Str)] string root_path,
                string[] patterns,
                uint patterns_len
            );

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr rpkg_scan_files_block_pkg(
                [MarshalAs(UnmanagedType.LPUTF8Str)] string root_path,
                string[] patterns,
                uint patterns_len
            );

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr rpkg_scan_files_block_manifest(
                [MarshalAs(UnmanagedType.LPUTF8Str)] string root_path,
                string[] patterns,
                uint patterns_len
            );

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr rpkg_scan_files_block_pkg_manifest(
                [MarshalAs(UnmanagedType.LPUTF8Str)] string root_path,
                string[] patterns,
                uint patterns_len
            );

        // ============================================================
        // BuildMap api
        // ============================================================
        // return BuildMap ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_new([MarshalAs(UnmanagedType.LPUTF8Str)] string root_path);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern bool bm_insert(
                IntPtr ptr,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string addon_path,
                string[] patterns,
                uint patterns_len
            );

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern void bm_dispose(IntPtr ptr);

        // return Dependencies ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_resolve_bundle_deps(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path);

        // return Dependencies ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_resolve_subscene_deps(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path);

        // return Dependencies ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_resolve_dylib_deps(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path);

        // return Dependencies ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_resolve_file_deps(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path);

        // return Dependencies ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_resolve_zip_deps(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_asset_urls(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string addon_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_bundle_paths(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string addon_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_subscene_paths(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string addon_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_file_paths(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string addon_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_dylib_paths(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string addon_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_zip_paths(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string addon_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_bundle_paths_from_pkg(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string pkg_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_subscene_paths_from_pkg(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string pkg_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_file_paths_from_pkg(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string pkg_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_dylib_paths_from_pkg(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string pkg_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_zip_paths_from_pkg(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string pkg_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_bundle_assets(IntPtr ptr,[MarshalAs(UnmanagedType.LPUTF8Str)] string target_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_subscene_assets(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_file_assets(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_dylib_assets(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_get_zip_assets(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern string bm_get_root_path(IntPtr ptr);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern string bm_find_bundle_url(IntPtr ptr, string bundle_path);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern string bm_debug_info(IntPtr ptr);

        // ============================================================
        // Dependencies api
        // ============================================================
        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr dependencies_get_targets(IntPtr ptr);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern bool dependencies_is_circular(IntPtr ptr);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern void dependencies_dispose(IntPtr ptr);

        // ============================================================
        // Vec<String> api
        // ============================================================

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern uint strs_len(IntPtr ptr);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern string strs_get(IntPtr ptr, uint index);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern void strs_dispose(IntPtr ptr);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern string try_log_once();


        public static void PkgMatchTest()
        {
            UnityBuildTest();

        }

        private static void UnityBuildTest()
        {
            Console.WriteLine("[rpkg]");
            Console.WriteLine("  version: " + get_version());
            Console.WriteLine("");

            // 搜索所有 pkg 文件(不是 build map 的步骤)
            string asset_path = "../../../../../tests/pkg-dependencies/BuildAssets";
            string[] pkg_patterns = { "**/.pkg" };
            IntPtr pkgs_ptr = rpkg_scan_files(asset_path, pkg_patterns, (UInt32)pkg_patterns.Length);
            Console.WriteLine("total pkgs:");

            string[] total_pkgs = Ptr2StringList(pkgs_ptr);
            foreach (string item in total_pkgs)
            {
                Console.WriteLine("  " + item);
            }
            Console.WriteLine("");

            // 初始化 BuildMap
            string root_path = "../../../../../tests/pkg-dependencies";
            IntPtr build_map_ptr = bm_new(root_path);

            // manifest.toml 解析获取 members
            Console.WriteLine("addons and pkgs:");
            string[] members = { "./", "addon1", "addon2" };

            // 获取 member 下的 pkg 文件
            foreach (string member in members)
            {
                 string addon_path = asset_path +"/"+ member;
                IntPtr addon_pkgs_ptr = rpkg_scan_files_block_manifest(addon_path, pkg_patterns, (UInt32)pkg_patterns.Length);
                Console.WriteLine("  addon " + member + " pkgs (" + addon_path + "):");

                string[] addon_pkgs = Ptr2StringList(addon_pkgs_ptr);
                foreach (string item in addon_pkgs)
                {
                    Console.WriteLine("    " + item);
                }

                // 插入 member 及其 pkgs 至 build_map
                bool is_insert_succ = bm_insert(build_map_ptr, addon_path, addon_pkgs, (UInt32)addon_pkgs.Length);
                if (!is_insert_succ)
                {
                    Console.WriteLine("build map insert failed: " + addon_path);
                    break;
                }
            }
            Console.WriteLine("");
            Console.WriteLine("build map: ");
            Console.WriteLine(bm_debug_info(build_map_ptr));
            Console.WriteLine("");


            // 获取 pkg 文件里的某个 bundle
            string bundle_path = "BuildAssets/addon1/Prefab";
            string addon_path1 = "../../../../../tests/pkg-dependencies/BuildAssets/addon1";
            string bundle_url = bm_find_bundle_url(build_map_ptr, bundle_path);
            Console.WriteLine(bundle_path + "(" + bundle_url + ")" + " deps");

            // 获取这个 bundle 的依赖（含自身）
            IntPtr deps_ptr = bm_resolve_bundle_deps(build_map_ptr, bundle_path);
            IntPtr to_build_ptr = dependencies_get_targets(deps_ptr);
            string[] to_build = Ptr2StringList(to_build_ptr);
            foreach (string target_path in to_build)
            {
                Console.WriteLine("  " + target_path + " assets:");

                // 获取这个 bundle 关联的具体资源
                IntPtr asset_paths_ptr = bm_get_bundle_assets(build_map_ptr, target_path);
                foreach(string path in Ptr2StringList(asset_paths_ptr))
                {
                    Console.WriteLine("    path: " + path);
                }
            }
            Console.WriteLine();

            Console.WriteLine("asset_urls:");
            IntPtr asset_urls_ptr = bm_get_asset_urls(build_map_ptr, addon_path1);
            foreach (string url in Ptr2StringList(asset_urls_ptr))
            {
                Console.WriteLine("  url: " + url);
            }



            // 获取错误 bundle 并提示信息
            {
                string err_bundle_path = "BuildAssets/addon1/Prefab/A";
                string url = bm_find_bundle_url(build_map_ptr, err_bundle_path);
                if (string.IsNullOrEmpty(url))
                {
                    Console.WriteLine("error:");
                    Console.WriteLine(try_log_once());
                }
            }

            dependencies_dispose(deps_ptr);
            bm_dispose(build_map_ptr);
        }

        private static string[] Ptr2StringList(System.IntPtr strs_ptr)
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
