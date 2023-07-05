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
        public static extern IntPtr bm_init(
                [MarshalAs(UnmanagedType.LPUTF8Str)] string root_path,
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

        // return Assets ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_scan_bundle_assets(
                IntPtr ptr,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string mount_path,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path
            );

        // return Assets ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_scan_subscene_assets(
                IntPtr ptr,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string mount_path,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path
            );

        // return Assets ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_scan_dylib_assets(
                IntPtr ptr,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string mount_path,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path
            );

        // return Assets ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_scan_file_assets(
                IntPtr ptr,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string mount_path,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path
            );

        // return Assets ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr bm_scan_zip_assets(
                IntPtr ptr,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string mount_path,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string target_path
            );

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern string bm_get_root_path(IntPtr ptr);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern string bm_debug_info(IntPtr ptr);

        // ============================================================
        // Dependencies api
        // ============================================================
        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr dependencies_get_targets(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr dependencies_get_invalid_targets(IntPtr ptr);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern bool dependencies_is_circular(IntPtr ptr);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern void dependencies_dispose(IntPtr ptr);

        // ============================================================
        // Assets api
        // ============================================================
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern void assets_dispose(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr assets_get_paths(IntPtr ptr);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr assets_get_urls(IntPtr ptr);


        // ============================================================
        // Vec<String> api
        // ============================================================

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern uint strs_len(IntPtr ptr);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern string strs_get(IntPtr ptr, uint index);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern void strs_dispose(IntPtr ptr);

        public static void PkgMatchTest()
        {
            UnityBuildTest();

        }

        private static void UnityBuildTest()
        {
            Console.WriteLine("[rpkg]");
            Console.WriteLine("  version: " + get_version());
            Console.WriteLine("");

            // 搜索所有 pkg 文件
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
            IntPtr build_map_ptr = bm_init(root_path, total_pkgs, (UInt32)total_pkgs.Length);
            Console.WriteLine("build map: ");
            Console.WriteLine(bm_debug_info(build_map_ptr));

            // manifest.toml 解析获取 members
            Console.WriteLine("addons and pkgs:");
            string[] members = { "./", "addon1", "addon2" };

            // 获取 addon 下的 pkg 文件
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
            }
            Console.WriteLine("");

            // 获取 pkg 文件里的某个 bundle
            string bundle_path = "BuildAssets/addon1/Prefab";
            string mount_path = "../../../../../tests/pkg-dependencies/BuildAssets/addon1";
            Console.WriteLine(bundle_path + " deps");

            // 获取这个 bundle 的依赖（含自身）
            IntPtr deps_ptr = bm_resolve_bundle_deps(build_map_ptr, bundle_path);
            IntPtr to_build_ptr = dependencies_get_targets(deps_ptr);
            string[] to_build = Ptr2StringList(to_build_ptr);
            foreach (string target in to_build)
            {
                Console.WriteLine("  " + target + " assets:");

                // 获取这个 bundle 关联的具体资源
                IntPtr assets_ptr = bm_scan_bundle_assets(build_map_ptr, mount_path, target);
                IntPtr asset_paths_ptr = assets_get_paths(assets_ptr);
                IntPtr asset_urls_ptr = assets_get_urls(assets_ptr);
                string[] asset_paths = Ptr2StringList(asset_paths_ptr);
                string[] asset_urls = Ptr2StringList(asset_urls_ptr);
                for (int i = 0; i < asset_paths.Length; i++)
                {
                    Console.WriteLine("    path: " + asset_paths[i] + ", url: " + asset_urls[i]);
                }
                assets_dispose(assets_ptr);             
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
