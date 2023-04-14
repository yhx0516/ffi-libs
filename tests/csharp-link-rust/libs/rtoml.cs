using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

namespace csharp_link_rust.libs
{
    public class rtoml
    {
        // ===============================================
        // Info
        // ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern string get_version();

        /// ===============================================
        /// Document in toml
        /// ===============================================
        // return Document ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr parse_toml_file([MarshalAs(UnmanagedType.LPUTF8Str)] string url);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr parse_toml_str([MarshalAs(UnmanagedType.LPUTF8Str)] string url);

        // return Item ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr get_from_document(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr as_item_from_document(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr as_table_from_document(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void dispose_document(IntPtr ptr);

        /// ===============================================
        /// Item in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_value_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr as_value_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_table_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr as_table_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_array_of_tables_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr as_array_of_tables_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_none_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_integer_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern long as_integer_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_float_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern float as_float_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_bool_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool as_bool_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_str_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern string as_str_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_array_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr as_array_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_inline_array_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr as_inline_table_from_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void dispose_item(IntPtr ptr);

        /// ===============================================
        /// Value in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern string type_name_from_value(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_integer_from_value(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern long as_integer_from_value(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_float_from_value(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern float as_float_from_value(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_bool_from_value(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool as_bool_from_value(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_str_from_value(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern string as_str_from_value(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_array_from_value(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr as_array_from_value(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_inline_array_from_value(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr as_inline_table_from_value(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void dispose_value(IntPtr ptr);

        /// ===============================================
        /// Array in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_empty_from_array(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern uint len_from_array(IntPtr ptr);

        // return Value ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr get_from_array(IntPtr ptr, uint index);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void dispose_array(IntPtr ptr);

        /// ===============================================
        /// Table in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_empty_from_table(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern uint len_from_table(IntPtr ptr);

        // return Item ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr get_from_table(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool contains_key_from_table(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool contains_table_from_table(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool contains_value_from_table(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool contains_array_of_tables_from_table(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void dispose_table(IntPtr ptr);
        /// ===============================================
        /// InlineTable in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_empty_from_inline_table(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern uint len_inline_table(IntPtr ptr);

        // return Value ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr get_from_inline_table(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool contains_key_from_inline_table(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void dispose_inline_table(IntPtr ptr);

        /// ===============================================
        /// ArrayOfTables in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool is_empty_from_table_array(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern uint len_from_table_array(IntPtr ptr);

        // return Value ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr get_from_table_array(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void dispose_table_array(IntPtr ptr);

        // parse toml test
        public static void ParseTomlTest()
        {
            Console.WriteLine("------------------  parse toml version ------------------");
            Console.WriteLine(get_version());
            Console.WriteLine("");

            Console.WriteLine("------------------ parse toml test ------------------");
            // System.IntPtr doc = parse_toml_file("../../../../pkg.toml");

            string context = System.IO.File.ReadAllText("../../../../pkg.toml");
            System.IntPtr doc = parse_toml_str(context);

            System.IntPtr item = get_from_document(doc, "output");
            string output = as_str_from_item(item);
            Console.WriteLine("output: " + output);
            Console.WriteLine("");
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
            Console.WriteLine("");

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
        }
    }
}
