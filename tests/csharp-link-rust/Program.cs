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
        // ===============================================
        // Info
        // ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern string get_version();

        // ===============================================
        // PKG Matcher
        // ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr pkg_match(
                [MarshalAs(UnmanagedType.LPUTF8Str)] string root_path,
                string[] patterns,
                UInt32 patterns_len
            );

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern UInt32 strs_len(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern string strs_get(System.IntPtr ptr, UInt32 index);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern string dispose_strs(System.IntPtr ptr);


        /// ===============================================
        /// Document in toml
        /// ===============================================
        // return Document ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr parse_toml_file([MarshalAs(UnmanagedType.LPUTF8Str)] string url);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr parse_toml_str([MarshalAs(UnmanagedType.LPUTF8Str)] string url);

        // return Item ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr get_from_document(System.IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr as_item_from_document(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr as_table_from_document(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void dispose_document(System.IntPtr ptr);

        /// ===============================================
        /// Item in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_value_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr as_value_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_table_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr as_table_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_array_of_tables_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr as_array_of_tables_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_none_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_integer_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern Int64 as_integer_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_float_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern float as_float_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_bool_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool as_bool_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_str_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern string as_str_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_array_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr as_array_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_inline_array_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr as_inline_table_from_item(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void dispose_item(System.IntPtr ptr);

        /// ===============================================
        /// Value in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern string type_name_from_value(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_integer_from_value(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern Int64 as_integer_from_value(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_float_from_value(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern float as_float_from_value(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_bool_from_value(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool as_bool_from_value(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_str_from_value(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern string as_str_from_value(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_array_from_value(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr as_array_from_value(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_inline_array_from_value(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr as_inline_table_from_value(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void dispose_value(System.IntPtr ptr);

        /// ===============================================
        /// Array in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_empty_from_array(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern UInt32 len_from_array(System.IntPtr ptr);

        // return Value ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr get_from_array(System.IntPtr ptr, UInt32 index);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void dispose_array(System.IntPtr ptr);

        /// ===============================================
        /// Table in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_empty_from_table(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern UInt32 len_from_table(System.IntPtr ptr);

        // return Item ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr get_from_table(System.IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool contains_key_from_table(System.IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool contains_table_from_table(System.IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool contains_value_from_table(System.IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool contains_array_of_tables_from_table(System.IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void dispose_table(System.IntPtr ptr);
        /// ===============================================
        /// InlineTable in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_empty_from_inline_table(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern UInt32 len_inline_table(System.IntPtr ptr);

        // return Value ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr get_from_inline_table(System.IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool contains_key_from_inline_table(System.IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void dispose_inline_table(System.IntPtr ptr);

        /// ===============================================
        /// ArrayOfTables in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_empty_from_table_array(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern UInt32 len_from_table_array(System.IntPtr ptr);

        // return Value ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr get_from_table_array(System.IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void dispose_table_array(System.IntPtr ptr);

        static void Main(string[] args)
        {
            get_version_test();

            // parse_toml_test();

            pkg_match_test();
        }

        public static void get_version_test()
        {
            Console.WriteLine(get_version());
        }

        public static void pkg_match_test()
        {

            string root_path1 = "../../../../../tests/pkg_assets";
            string[] patterns1 = { "**/foo1/*.asset", "**/foo1/*.toml" };
            foreach (string file in pkg_inner_match(root_path1, patterns1))
            {
                Console.WriteLine(file);
            }
            Console.WriteLine("");

            string root_path2 = "../../../../../tests/pkg_assets";
            string[] patterns2 = { "**/foo2/*.txt", "!**/foo2/bar/*2.txt" };
            foreach (string file in pkg_inner_match(root_path2, patterns2))
            {
                Console.WriteLine(file);
            }
            Console.WriteLine("");

            string root_path3 = "../../../../../tests/pkg_assets";
            string[] patterns3 = { "**/foo3/*.txt", "**/foo3/**/*.txt" };
            foreach (string file in pkg_inner_match(root_path3, patterns3))
            {
                Console.WriteLine(file);
            }
            Console.WriteLine("");

        }

        public static string[] pkg_inner_match(string root_path, string[] patterns)
        {
            System.IntPtr strs_ptr = pkg_match(root_path, patterns, (UInt32)patterns.Length);

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

                    return files;
                }
                dispose_strs(strs_ptr);
            }
            return null;
        }
        // parse toml test
        public static void parse_toml_test()
        {

            // System.IntPtr doc = parse_toml_file("../../../../pkg.toml");

            string context = System.IO.File.ReadAllText("../../../../pkg.toml");
            System.IntPtr doc = parse_toml_str(context);

            System.IntPtr item = get_from_document(doc, "output");
            string output = as_str_from_item(item);
            Console.WriteLine("output: " + output);
            Console.WriteLine("\n");
            dispose_item(item);


            System.IntPtr bundles_item = get_from_document(doc, "bundles");
            System.IntPtr bundles_table = as_table_from_item(bundles_item);

            System.IntPtr includes_item = get_from_table(bundles_table, "includes");
            System.IntPtr includes_array = as_array_from_item(includes_item);
            List<string> includes = new List<string>();
            for (uint i = 0; i < len_from_array(includes_array); i++)
            {
                System.IntPtr val_ptr = get_from_array(includes_array, i);
                string str = as_str_from_value(val_ptr);
                includes.Add(str);

                Console.WriteLine("include: " + str);
                dispose_value(val_ptr);
            }
            Console.WriteLine("includes len: " + includes.Count);
            Console.WriteLine("\n");

            dispose_array(includes_array);
            dispose_item(includes_item);

            System.IntPtr ignores_item = get_from_table(bundles_table, "ignores");
            System.IntPtr ignores_array = as_array_from_item(ignores_item);
            List<string> ignores = new List<string>();
            for (uint i = 0; i < len_from_array(ignores_array); i++)
            {
                System.IntPtr val_ptr = get_from_array(ignores_array, i);
                string str = as_str_from_value(val_ptr);
                ignores.Add(str);

                Console.WriteLine("ignore: " + str);
                dispose_value(val_ptr);
            }
            Console.WriteLine("ignores len: " + ignores.Count);

            dispose_array(ignores_array);
            dispose_item(includes_item);

            dispose_table(bundles_table);
            dispose_item(bundles_item);
            dispose_document(doc);

            GC.Collect();
            GC.WaitForPendingFinalizers();
            Console.ReadLine();
        }  
    }
}

