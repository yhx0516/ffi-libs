using System;
using System.Collections.Generic;
using System.Numerics;
using System.Runtime.InteropServices;
using static System.Net.Mime.MediaTypeNames;

namespace csharp_link_rust
{
    internal class Program
    {
        /// ===============================================
        /// Document in toml
        /// ===============================================
        // return Document ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr parse_toml([MarshalAs(UnmanagedType.LPUTF8Str)] string url);

        // return Item ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr get_from_document(System.IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr as_item_from_document(System.IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern System.IntPtr as_table_from_document(System.IntPtr ptr);

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
        public static extern bool contains_key_from_table_array(System.IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        static void Main(string[] args)
        {
            System.IntPtr doc = parse_toml("../../../../pkg.toml");

            System.IntPtr item = get_from_document(doc, "output");
            string output = as_str_from_item(item);
            Console.WriteLine("output: " + output);
            Console.WriteLine("\n");

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
            }
            Console.WriteLine("includes len: " + includes.Count);
            Console.WriteLine("\n");

            System.IntPtr ignores_item = get_from_table(bundles_table, "ignores");
            System.IntPtr ignores_array = as_array_from_item(ignores_item);
            List<string> ignores = new List<string>();
            for (uint i = 0; i < len_from_array(ignores_array); i++)
            {
                System.IntPtr val_ptr = get_from_array(ignores_array, i);
                string str = as_str_from_value(val_ptr);
                ignores.Add(str);
                Console.WriteLine("ignore: " + str);
            }
            Console.WriteLine("ignores len: " + ignores.Count);

            Console.ReadLine();
        }
    }
}

