using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.InteropServices;
using System.Text;

namespace csharp_link_rust.libs
{
    public class rpkg
    {
        #if !UNITY_EDITOR && UNITY_IPHONE
            const string dllName = "__Internal";
        #else
            const string dllName = "../../../../../target/debug/rpkg";
        #endif

        // ===============================================
        // Info
        // ===============================================
        [DllImport(dllName)]
        public static extern string get_version();

        // ===============================================
        // Scan api
        // ===============================================
        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr rpkg_scan_files(
                [MarshalAs(UnmanagedType.LPUTF8Str)] string root_path,
                string[] patterns,
                uint patterns_len
            );

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr rpkg_scan_files_rel_path(
                [MarshalAs(UnmanagedType.LPUTF8Str)] string root_path,
                string[] patterns,
                uint patterns_len
            );

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr rpkg_scan_files_block_by_pkg(
                [MarshalAs(UnmanagedType.LPUTF8Str)] string root_path,
                string[] patterns,
                uint patterns_len
            );

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr rpkg_scan_files_block_by_manifest(
                [MarshalAs(UnmanagedType.LPUTF8Str)] string root_path,
                string[] patterns,
                uint patterns_len
            );

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr rpkg_scan_files_block_by_pkg_manifest(
                [MarshalAs(UnmanagedType.LPUTF8Str)] string root_path,
                string[] patterns,
                uint patterns_len
            );

        // ============================================================
        // BuildMap api
        // ============================================================
        // return BuildMap ptr
        [DllImport(dllName)]
        public static extern IntPtr bm_new([MarshalAs(UnmanagedType.LPUTF8Str)] string root_path);

        [DllImport(dllName)]
        public static extern bool bm_insert(
                IntPtr ptr,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string addon_path,
                string[] patterns,
                uint patterns_len
            );

        [DllImport(dllName)]
        public static extern void bm_dispose(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr bm_get_target_types(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string addon_path);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr bm_get_target_types_from_pkg(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string pkg_path);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr bm_get_target_paths(
                IntPtr ptr,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string addon_path,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string target_type
            );

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr bm_get_target_paths_from_pkg(
                IntPtr ptr,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string pkg_path,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string target_type
            );

        // return BuildCollection ptr
        [DllImport(dllName)]
        public static extern IntPtr bm_seek_build_collection(
                IntPtr ptr,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string select_path
            );

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr bm_get_addon_pkg_paths(
                IntPtr ptr,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string pkg_path
            );

        // return Dependencies ptr
        [DllImport(dllName)]
        public static extern IntPtr bm_resolve_target_deps(
                IntPtr ptr,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string target_type
            );

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr bm_get_target_assets(
                IntPtr ptr,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string target_type
            );

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr bm_get_asset_urls(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string addon_path);


        [DllImport(dllName)]
        public static extern string bm_get_root_path(IntPtr ptr);

        [DllImport(dllName)]
        public static extern string bm_find_bundle_path(IntPtr ptr, string target_path);

        [DllImport(dllName)]
        public static extern string bm_debug_info(IntPtr ptr);

        // ============================================================
        // Dependencies api
        // ============================================================
        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr dependencies_get_targets(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool dependencies_is_circular(IntPtr ptr);

        [DllImport(dllName)]
        public static extern void dependencies_dispose(IntPtr ptr);

        // ============================================================
        // BuildCollection api
        // ============================================================
        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr bc_get_addon_paths(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr bc_get_pkg_paths(IntPtr ptr);

        [DllImport(dllName)]
        public static extern void bc_dispose(IntPtr ptr);

        // ============================================================
        // Vec<String> api
        // ============================================================

        [DllImport(dllName)]
        public static extern uint strs_len(IntPtr ptr);

        [DllImport(dllName)]
        public static extern string strs_get(IntPtr ptr, uint index);

        [DllImport(dllName)]
        public static extern void strs_dispose(IntPtr ptr);

        [DllImport(dllName)]
        public static extern string try_get_err();


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
                IntPtr addon_pkgs_ptr = rpkg_scan_files_block_by_manifest(addon_path, pkg_patterns, (UInt32)pkg_patterns.Length);
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

            IntPtr addon_pkg_paths_ptr = bm_get_addon_pkg_paths(build_map_ptr, addon_path1);
            string[] addon_pkg_paths = Ptr2StringList(addon_pkg_paths_ptr);
            Console.WriteLine($"{addon_path1} pkg_paths: ");

            foreach (string pkg_path in addon_pkg_paths) {
                Console.WriteLine($"    {pkg_path}");
            }

            // 收集指定目录 addon 与 pkg 的信息
            string[] select_paths = {
                "../../../../../tests/pkg-dependencies",
                "../../../../../tests/pkg-dependencies/BuildAssets",
                "../../../../../tests/pkg-dependencies/BuildAssets/addon1",
                "../../../../../tests/pkg-dependencies/BuildAssets/addon2",
                "../../../../../tests/pkg-dependencies/BuildAssets/manifest.toml",
                "../../../../../tests/pkg-dependencies/BuildAssets/addon1/manifest.toml",
                "../../../../../tests/pkg-dependencies/BuildAssets/addon1/Prefab/.pkg",
                "../../../../../tests/pkg-dependencies/CircularDep",
                "../../../../../tests/pkg-dependencies/CircularDep/A/.pkg",
            };

            foreach (string select_path in select_paths) {
                IntPtr build_collection_ptr = bm_seek_build_collection(build_map_ptr, select_path);

                Console.WriteLine("  select_path:" + select_path);
                Console.WriteLine("     seek addon paths");

                IntPtr addon_paths_ptr = bc_get_addon_paths(build_collection_ptr);
                string[] addon_paths = Ptr2StringList(addon_paths_ptr);
                foreach (string addon_path in addon_paths) {
                    Console.WriteLine("        " + addon_path);
                }

                Console.WriteLine("     seek pkg paths");
                IntPtr pkg_paths_ptr = bc_get_pkg_paths(build_collection_ptr);
                string[] pkg_paths = Ptr2StringList(pkg_paths_ptr);
                foreach (string pkg_path in pkg_paths) {
                    Console.WriteLine("    " + pkg_path);
                }

                bc_dispose(build_collection_ptr);
            }


            string bundle_url = bm_find_bundle_path(build_map_ptr, bundle_path);
            Console.WriteLine(bundle_path + "(" + bundle_url + ")" + " deps");

            // 获取所有 target_type
            IntPtr target_types_ptr = bm_get_target_types(build_map_ptr, addon_path1);
            string[] target_types = Ptr2StringList(target_types_ptr);
            foreach (string target_type in target_types)
            {
                // 获取改类型的所有 target_path
                IntPtr target_paths_ptr = bm_get_target_paths(build_map_ptr, addon_path1, target_type);
                string[] target_paths = Ptr2StringList(target_paths_ptr);

                foreach(string target_path in target_paths)
                {
                    // 获取这个 bundle 的依赖（不含自身）
                    IntPtr deps_ptr = bm_resolve_target_deps(build_map_ptr, bundle_path, target_type);
                    IntPtr to_build_ptr = dependencies_get_targets(deps_ptr);
                    string[] to_build = Ptr2StringList(to_build_ptr);
                    foreach (string to_build_path in to_build)
                    {
                        Console.WriteLine("  " + target_path + " assets:");

                        // 获取这个 bundle 关联的具体资源
                        IntPtr asset_paths_ptr = bm_get_target_assets(build_map_ptr, to_build_path, target_type);
                        foreach (string path in Ptr2StringList(asset_paths_ptr))
                        {
                            Console.WriteLine("    path: " + path);
                        }
                    }
                    Console.WriteLine();
                    dependencies_dispose(deps_ptr);
                }
            }

            Console.WriteLine("asset_urls:");
            IntPtr asset_urls_ptr = bm_get_asset_urls(build_map_ptr, addon_path1);
            foreach (string url in Ptr2StringList(asset_urls_ptr))
            {
                Console.WriteLine("  url: " + url);
            }

            // 获取错误 bundle 并提示信息
            {
                string err_bundle_path = "BuildAssets/addon1/Prefab/A";
                string url = bm_find_bundle_path(build_map_ptr, err_bundle_path);
                if (string.IsNullOrEmpty(url))
                {
                    Console.WriteLine("error:");
                    Console.WriteLine(try_get_err());
                }
            }

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
