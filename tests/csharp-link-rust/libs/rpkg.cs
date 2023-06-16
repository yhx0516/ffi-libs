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
        // Pkg Match Patterns
        // ===============================================

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr pkg_match_patterns(
                [MarshalAs(UnmanagedType.LPUTF8Str)] string root_path,
                string[] patterns,
                uint patterns_len
            );

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern uint strs_len(IntPtr ptr);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern string strs_get(IntPtr ptr, uint index);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern void dispose_strs(IntPtr ptr);

        // ============================================================
        // PKG Seek Dependencies
        // ============================================================
        // return Dependencies ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr pkg_seek_dependencies(
                [MarshalAs(UnmanagedType.LPUTF8Str)] string root_path,
                [MarshalAs(UnmanagedType.LPUTF8Str)] string file,
                string[] patterns,
                uint patterns_len
            );

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr dependencies_get_files(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr dependencies_get_invalid_files(IntPtr ptr);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern bool dependencies_is_circular(IntPtr ptr);

        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern void dependencies_dispose(IntPtr ptr);

        public static void PkgMatchTest()
        {
            try
            {
                PkgMatchPattern();
            }
            catch (Exception)
            {
                Console.WriteLine("本部分测试需要配合 rust 生成的文件目录才能正常运行。");
            }       

            PkgSeekDependencies();
        }

        private static void PkgSeekDependencies()
        {
            Console.WriteLine("  - pkg seek dependencies");
            string root_path = "../../../../../tests";
            string path1 = "../../../../../tests/pkg-dependencies/BuildAssets/Prefab/.pkg";
            string [] pattterns1 = {
                    "/pkg-dependencies/BuildAssets/Material/.pkg",
                    "/pkg-dependencies/BuildAssets/Material/DepMaterial/.pkg",
            };
            IntPtr deps_ptr1 = pkg_seek_dependencies(root_path, path1, pattterns1, (UInt32)pattterns1.Length);
            System.IntPtr strs_ptr1 = dependencies_get_files(deps_ptr1);
            Console.WriteLine("      [" + path1 + "]");
            Console.WriteLine("      is_circular: " + dependencies_is_circular(deps_ptr1));
            Console.WriteLine("      res:");
            foreach (string file in InnerPrintStrs(strs_ptr1))
            {
                Console.WriteLine("        " + file);
            }
            dependencies_dispose(deps_ptr1);
            Console.WriteLine("");

            string path2 = "../../../../../tests/pkg-dependencies/BuildAssets/rel.pkg";
            string[] pattterns2 = { "**/.pkg" };
            IntPtr deps_ptr2 = pkg_seek_dependencies(root_path, path2, pattterns2, (UInt32)pattterns2.Length);
            System.IntPtr strs_ptr2 = dependencies_get_files(deps_ptr2);
            Console.WriteLine("      [" + path2 + "]");
            Console.WriteLine("      is_circular: " + dependencies_is_circular(deps_ptr2));
            Console.WriteLine("      res:");
            foreach (string file in InnerPrintStrs(strs_ptr2))
            {
                Console.WriteLine("        " + file);
            }
            dependencies_dispose(deps_ptr2);
            Console.WriteLine("");

            string path3 = "../../../../../tests/pkg-dependencies/BuildAssets/rel2.pkg";
            string[] pattterns3 = { "**/PKGTest/.pkg" };
            IntPtr deps_ptr3 = pkg_seek_dependencies(root_path, path3, pattterns3, (UInt32)pattterns3.Length);
            System.IntPtr strs_ptr3 = dependencies_get_files(deps_ptr3);
            Console.WriteLine("      [" + path3 + "]");
            Console.WriteLine("      is_circular: " + dependencies_is_circular(deps_ptr3));
            Console.WriteLine("      res:");
            foreach (string file in InnerPrintStrs(strs_ptr3))
            {
                Console.WriteLine("        " + file);
            }
            dependencies_dispose(deps_ptr3);
            Console.WriteLine("");

            string path4 = "../../../../../tests/pkg-dependencies/CircularDep/A/.pkg";
            string[] pattterns4 = { "/pkg-dependencies/CircularDep/B/.pkg" };
            IntPtr deps_ptr4 = pkg_seek_dependencies(root_path, path4, pattterns4, (UInt32)pattterns4.Length);
            System.IntPtr strs_ptr4 = dependencies_get_files(deps_ptr4);
            Console.WriteLine("      [" + path4 + "]");
            Console.WriteLine("      is_circular: " + dependencies_is_circular(deps_ptr4));
            Console.WriteLine("      res:");
            foreach (string file in InnerPrintStrs(strs_ptr4))
            {
                Console.WriteLine("        " + file);
            }
            dependencies_dispose(deps_ptr4);
            Console.WriteLine("");
        }



        // NOTE: 基于 rust pkg_match_files_test() 生成的文件树结构进行测试
        private static void PkgMatchPattern()
        {
            Console.WriteLine("[rpkg]");
            Console.WriteLine("  - pkg match version");
            Console.WriteLine("      version: " + get_version());
            Console.WriteLine("");

            Console.WriteLine("  - pkg match patterns test");
            string root_path1 = "../../../../../target/tmp/pkg_assets/foo1";
            string[] patterns1 = { "*.asset" };
            System.IntPtr strs_ptr1 = pkg_match_patterns(root_path1, patterns1, (UInt32)patterns1.Length);
            Console.WriteLine("      [\"*.asset\"]: ");
            foreach (string file in InnerPrintStrs(strs_ptr1))
            {
                Console.WriteLine("        " + file);
            }
            Console.WriteLine("");

            string root_path2 = "../../../../../target/tmp/pkg_assets/foo2";
            string[] patterns2 = { "*.txt", "!bar/*2.txt" };
            System.IntPtr strs_ptr2 = pkg_match_patterns(root_path2, patterns2, (UInt32)patterns2.Length);
            Console.WriteLine("      [\"*.txt\", \"!bar/*2.txt\"]: ");
            foreach (string file in InnerPrintStrs(strs_ptr2))
            {
                Console.WriteLine("        " + file);
            }
            Console.WriteLine("");

            string root_path3 = "../../../../../target/tmp/pkg_assets/foo3";
            string[] patterns3 = { "*.txt", "**/*.txt" };
            System.IntPtr strs_ptr3 = pkg_match_patterns(root_path3, patterns3, (UInt32)patterns3.Length);
            Console.WriteLine("      [\"*.txt\", \"**/*.txt\"]: ");
            foreach (string file in InnerPrintStrs(strs_ptr3))
            {
                Console.WriteLine("        " + file);
            }
            Console.WriteLine("");
        }

        private static string[] InnerPrintStrs(System.IntPtr strs_ptr)
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

                    dispose_strs(strs_ptr);
                    return files;
                }
                dispose_strs(strs_ptr);
            }
            return new string[0];
        }
    }
}
