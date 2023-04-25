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
        public static extern IntPtr document_parse_file([MarshalAs(UnmanagedType.LPUTF8Str)] string url);

        // return Document ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr document_parse_content([MarshalAs(UnmanagedType.LPUTF8Str)] string url);

        // return Item ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr document_get(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr document_as_item(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr document_as_table(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void document_dispose(IntPtr ptr);

        /// ===============================================
        /// Item in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool item_is_value(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr item_as_value(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool item_is_table(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr item_as_table(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool item_is_array_of_tables(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr item_as_array_of_tables(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool item_is_none(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool item_is_integer(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern int item_as_int32(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern long item_as_int64(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool item_is_float(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern float item_as_float(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern double item_as_double(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool item_is_bool(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool item_as_bool(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool item_is_str(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern string item_as_str(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool item_is_array(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr item_as_array(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool item_is_inline_array(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr item_as_inline_table(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void item_dispose(IntPtr ptr);

        /// ===============================================
        /// Value in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern string value_type_name(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool value_is_integer(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern int value_as_int32(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern long value_as_int64(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool value_is_float(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern float value_as_float(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern double value_as_double(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool value_is_bool(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool value_as_bool(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool value_is_str(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern string value_as_str(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool value_is_array(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr value_as_array(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool value_is_inline_array(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr value_as_inline_table(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void value_dispose(IntPtr ptr);

        /// ===============================================
        /// Array in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool array_is_empty(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern uint array_len(IntPtr ptr);

        // return Value ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr array_get(IntPtr ptr, uint index);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void array_dispose(IntPtr ptr);

        /// ===============================================
        /// Table in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool table_is_empty(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern uint table_len(IntPtr ptr);

        // return Item ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr table_get(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool table_contains_key(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool table_contains_table(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool table_contains_value(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool table_contains_array_of_tables(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void table_dispose(IntPtr ptr);
        /// ===============================================
        /// InlineTable in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool inline_table_is_empty(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern uint inline_table_len(IntPtr ptr);

        // return Value ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr inline_table_get(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool inline_table_contains_key(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void inline_table_dispose(IntPtr ptr);

        /// ===============================================
        /// ArrayOfTables in toml
        /// ===============================================
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern bool table_array_is_empty(IntPtr ptr);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern uint table_array_len(IntPtr ptr);

        // return Value ptr
        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern IntPtr table_array_get(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport("../../../../../target/debug/rtoml.dll")]
        public static extern void table_array_dispose(IntPtr ptr);

        // parse toml test
        public static void ParseTomlTest()
        {
            Console.WriteLine("------------------  parse toml version ------------------");
            Console.WriteLine(get_version());
            Console.WriteLine("");

            Console.WriteLine("------------------ parse toml test ------------------");
            // System.IntPtr doc = parse_toml_file("../../../../pkg.toml");

            string context = System.IO.File.ReadAllText("../../../../pkg.toml");
            System.IntPtr doc = document_parse_content(context);

            System.IntPtr item_str = document_get(doc, "str_val");
            string str_val = item_as_str(item_str);
            Console.WriteLine("   str val: " + str_val);
            item_dispose(item_str);

            System.IntPtr item_int32 = document_get(doc, "int32_val");
            int int32_val = item_as_int32(item_int32);
            Console.WriteLine(" int32 val: " + int32_val);

            System.IntPtr item_int64 = document_get(doc, "int64_val");
            long int64_val = item_as_int64(item_int64);
            Console.WriteLine(" int64 val: " + int64_val);

            System.IntPtr item_float = document_get(doc, "float_val");
            float float_val = item_as_float(item_float);
            Console.WriteLine(" float val: " + float_val);

            System.IntPtr item_double = document_get(doc, "double_val");
            double double_val = item_as_double(item_double);
            Console.WriteLine("double val: " + double_val);
            Console.WriteLine("");

            System.IntPtr bundles_item = document_get(doc, "bundles");
            System.IntPtr bundles_table = item_as_table(bundles_item);

            System.IntPtr includes_item = table_get(bundles_table, "includes");
            System.IntPtr includes_array = item_as_array(includes_item);
            List<string> includes = new List<string>();
            for (uint i = 0; i < array_len(includes_array); i++)
            {
                System.IntPtr val_ptr = array_get(includes_array, i);
                string str = value_as_str(val_ptr);
                includes.Add(str);

                Console.WriteLine("include: " + str);
                value_dispose(val_ptr);
            }
            Console.WriteLine("includes len: " + includes.Count);
            Console.WriteLine("");

            array_dispose(includes_array);
            item_dispose(includes_item);

            System.IntPtr ignores_item = table_get(bundles_table, "ignores");
            System.IntPtr ignores_array = item_as_array(ignores_item);
            List<string> ignores = new List<string>();
            for (uint i = 0; i < array_len(ignores_array); i++)
            {
                System.IntPtr val_ptr = array_get(ignores_array, i);
                string str = value_as_str(val_ptr);
                ignores.Add(str);

                Console.WriteLine("ignore: " + str);
                value_dispose(val_ptr);
            }
            Console.WriteLine("ignores len: " + ignores.Count);

            array_dispose(ignores_array);
            item_dispose(includes_item);

            table_dispose(bundles_table);
            item_dispose(bundles_item);
            document_dispose(doc);

            GC.Collect();
            GC.WaitForPendingFinalizers();
        }
    }
}
