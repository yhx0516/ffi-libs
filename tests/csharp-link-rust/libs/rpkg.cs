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
        // Pkg Matcher
        // ===============================================
        [DllImport("../../../../../target/debug/rpkg.dll")]
        public static extern IntPtr pkg_match_file([MarshalAs(UnmanagedType.LPUTF8Str)] string root_path);

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

        // NOTE: 基于 rust pkg_match_files_test() 生成的文件树结构进行测试
        public static void PkgMatchTest()
        {
            Console.WriteLine("[rpkg]");
            Console.WriteLine("  - pkg match version");
            Console.WriteLine("      version: " + get_version());
            Console.WriteLine("");

            Console.WriteLine("  - pkg match file test");
            string file_path = "../../../../../target/tmp/pkg_assets/foo1/.pkg";
            System.IntPtr strs_ptr = pkg_match_file(file_path);
            Console.WriteLine("      res: ");
            foreach (string file in PkgInnerMatch(strs_ptr))
            {
                Console.WriteLine("        " + file);
            }
            Console.WriteLine("");

            Console.WriteLine("  - pkg match patterns test");
            string root_path1 = "../../../../../target/tmp/pkg_assets/foo1";
            string[] patterns1 = { "*.asset" };
            System.IntPtr strs_ptr1 = pkg_match_patterns(root_path1, patterns1, (UInt32)patterns1.Length);
            Console.WriteLine("      [\"*.asset\"]: ");
            foreach (string file in PkgInnerMatch(strs_ptr1))
            {
                Console.WriteLine("        " + file);
            }
            Console.WriteLine("");

            string root_path2 = "../../../../../target/tmp/pkg_assets/foo2";
            string[] patterns2 = { "*.txt", "!bar/*2.txt" };
            System.IntPtr strs_ptr2 = pkg_match_patterns(root_path2, patterns2, (UInt32)patterns2.Length);
            Console.WriteLine("      [\"*.txt\", \"!bar/*2.txt\"]: ");
            foreach (string file in PkgInnerMatch(strs_ptr2))
            {
                Console.WriteLine("        " + file);
            }
            Console.WriteLine("");

            string root_path3 = "../../../../../target/tmp/pkg_assets/foo3";
            string[] patterns3 = { "*.txt", "**/*.txt" };
            System.IntPtr strs_ptr3 = pkg_match_patterns(root_path3, patterns3, (UInt32)patterns3.Length);
            Console.WriteLine("      [\"*.txt\", \"**/*.txt\"]: ");
            foreach (string file in PkgInnerMatch(strs_ptr3))
            {
                Console.WriteLine("        " + file);
            }
        }

        private static string[] PkgInnerMatch(System.IntPtr strs_ptr)
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
