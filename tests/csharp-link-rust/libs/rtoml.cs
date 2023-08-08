using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.InteropServices;
using System.Text;

namespace csharp_link_rust.libs
{
    public class rtoml
    {
        #if !UNITY_EDITOR && UNITY_IPHONE
            const string dllName = "__Internal";
        #else
            const string dllName = "../../../../../target/debug/rtoml";
        #endif
        // ===============================================
        // Info
        // ===============================================
        // return c_char ptr
        [DllImport(dllName)]
        public static extern IntPtr get_version();

        /// ===============================================
        /// Document in toml
        /// ===============================================
        // return Document ptr
        [DllImport(dllName)]
        public static extern IntPtr document_parse_file([MarshalAs(UnmanagedType.LPUTF8Str)] string path);

        // return Document ptr
        [DllImport(dllName)]
        public static extern IntPtr document_parse_content([MarshalAs(UnmanagedType.LPUTF8Str)] string content);

        // return Item ptr
        [DllImport(dllName)]
        public static extern IntPtr document_get(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr document_get_keys(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr document_get_array_keys(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr document_get_table_keys(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr document_get_inline_table_keys(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr document_get_table_array_keys(IntPtr ptr);

        [DllImport(dllName)]
        public static extern IntPtr document_as_item(IntPtr ptr);

        [DllImport(dllName)]
        public static extern IntPtr document_as_table(IntPtr ptr);

        [DllImport(dllName)]
        public static extern void document_dispose(IntPtr ptr);

        // return Document ptr
        [DllImport(dllName)]
        public static extern IntPtr document_new();

        // item_ptr: Item ptr
        [DllImport(dllName)]
        public static extern bool document_insert(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key, IntPtr item_ptr);

        [DllImport(dllName)]
        public static extern bool document_remove(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport(dllName)]
        public static extern void document_clear(IntPtr ptr);

        // retrun *const c_char
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr document_to_string(IntPtr ptr);

        /// ===============================================
        /// Item in toml
        /// ===============================================
        [DllImport(dllName)]
        public static extern bool item_is_value(IntPtr ptr);

        [DllImport(dllName)]
        public static extern IntPtr item_as_value(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool item_is_table(IntPtr ptr);

        [DllImport(dllName)]
        public static extern IntPtr item_as_table(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool item_is_array_of_tables(IntPtr ptr);

        [DllImport(dllName)]
        public static extern IntPtr item_as_array_of_tables(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool item_is_none(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool item_is_integer(IntPtr ptr);

        [DllImport(dllName)]
        public static extern int item_as_int32(IntPtr ptr);

        [DllImport(dllName)]
        public static extern long item_as_int64(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool item_is_float(IntPtr ptr);

        [DllImport(dllName)]
        public static extern float item_as_float(IntPtr ptr);

        [DllImport(dllName)]
        public static extern double item_as_double(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool item_is_bool(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool item_as_bool(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool item_is_str(IntPtr ptr);

        // return c_char ptr
        [DllImport(dllName)]
        public static extern IntPtr item_as_str(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool item_is_array(IntPtr ptr);

        [DllImport(dllName)]
        public static extern IntPtr item_as_array(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool item_is_inline_table(IntPtr ptr);

        [DllImport(dllName)]
        public static extern IntPtr item_as_inline_table(IntPtr ptr);

        [DllImport(dllName)]
        public static extern void item_dispose(IntPtr ptr);

        // return Item ptr
        [DllImport(dllName)]
        public static extern IntPtr item_from_i32(int val);

        // return Item ptr
        [DllImport(dllName)]
        public static extern IntPtr item_from_i64(long val);

        // return Item ptr
        [DllImport(dllName)]
        public static extern IntPtr item_from_float(float val);

        // return Item ptr
        [DllImport(dllName)]
        public static extern IntPtr item_from_double(double val);

        // return Item ptr
        [DllImport(dllName)]
        public static extern IntPtr item_from_bool(bool val);

        // return Item ptr
        [DllImport(dllName)]
        public static extern IntPtr item_from_str([MarshalAs(UnmanagedType.LPUTF8Str)] string str);

        // value_ptr: Value ptr
        // return Item ptr
        [DllImport(dllName)]
        public static extern IntPtr item_from_value(IntPtr value_ptr);

        // inline_table_ptr: InlineTable ptr
        // return Item ptr
        [DllImport(dllName)]
        public static extern IntPtr item_from_inline_table(IntPtr inline_table_ptr);

        // table_ptr: Table ptr
        // return Item ptr
        [DllImport(dllName)]
        public static extern IntPtr item_from_table(IntPtr table_ptr);

        // array_ptr: Array ptr
        // return Item ptr
        [DllImport(dllName)]
        public static extern IntPtr item_from_array(IntPtr arrar_ptr);

        // retrun *const c_char
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr item_to_string(IntPtr ptr);

        // ===============================================
        // Value in toml
        // ===============================================
        // return c_char ptr
        [DllImport(dllName)]
        public static extern IntPtr value_type_name(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool value_is_integer(IntPtr ptr);

        [DllImport(dllName)]
        public static extern int value_as_int32(IntPtr ptr);

        [DllImport(dllName)]
        public static extern long value_as_int64(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool value_is_float(IntPtr ptr);

        [DllImport(dllName)]
        public static extern float value_as_float(IntPtr ptr);

        [DllImport(dllName)]
        public static extern double value_as_double(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool value_is_bool(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool value_as_bool(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool value_is_str(IntPtr ptr);

        // return c_char ptr
        [DllImport(dllName)]
        public static extern IntPtr value_as_str(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool value_is_array(IntPtr ptr);

        [DllImport(dllName)]
        public static extern IntPtr value_as_array(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool value_is_inline_table(IntPtr ptr);

        [DllImport(dllName)]
        public static extern IntPtr value_as_inline_table(IntPtr ptr);

        [DllImport(dllName)]
        public static extern void value_dispose(IntPtr ptr);

        // return Value ptr
        [DllImport(dllName)]
        public static extern IntPtr value_from_i32(int val);

        // return Value ptr
        [DllImport(dllName)]
        public static extern IntPtr value_from_i64(long val);

        // return Value ptr
        [DllImport(dllName)]
        public static extern IntPtr value_from_float(float val);

        // return Value ptr
        [DllImport(dllName)]
        public static extern IntPtr value_from_double(double val);

        // return Value ptr
        [DllImport(dllName)]
        public static extern IntPtr value_from_bool(bool val);

        // return Value ptr
        [DllImport(dllName)]
        public static extern IntPtr value_from_str([MarshalAs(UnmanagedType.LPUTF8Str)] string str);

        // item_ptr: Item ptr
        // return Value ptr
        [DllImport(dllName)]
        public static extern IntPtr value_from_item(IntPtr item_ptr);

        // inline_table_ptr: InlineTable ptr
        // return Value ptr
        [DllImport(dllName)]
        public static extern IntPtr value_from_inline_table(IntPtr inline_table_ptr);

        // array_ptr: Array ptr
        // return Value ptr
        [DllImport(dllName)]
        public static extern IntPtr value_from_array(IntPtr arrar_ptr);

        // retrun *const c_char
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr value_to_string(IntPtr ptr);

        /// ===============================================
        /// Array in toml
        /// ===============================================
        [DllImport(dllName)]
        public static extern bool array_is_empty(IntPtr ptr);

        [DllImport(dllName)]
        public static extern uint array_len(IntPtr ptr);

        // return Value ptr
        [DllImport(dllName)]
        public static extern IntPtr array_get(IntPtr ptr, uint index);

        [DllImport(dllName)]
        public static extern void array_dispose(IntPtr ptr);

        // return Array ptr
        [DllImport(dllName)]
        public static extern IntPtr array_new();

        // value_ptr: Value ptr
        [DllImport(dllName)]
        public static extern void array_push(IntPtr ptr, IntPtr value_ptr);

        // value_ptr: Value ptr
        [DllImport(dllName)]
        public static extern void array_insert(IntPtr ptr, uint index, IntPtr value_ptr);

        // value_ptr: Value ptr
        [DllImport(dllName)]
        public static extern void array_replace(IntPtr ptr, uint index, IntPtr value_ptr);

        [DllImport(dllName)]
        public static extern void array_remove(IntPtr ptr, uint index);

        [DllImport(dllName)]
        public static extern void array_clear(IntPtr ptr);

        // retrun *const c_char
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr array_to_string(IntPtr ptr);

        [DllImport(dllName)]
        public static extern void array_pretty(IntPtr ptr);

        /// ===============================================
        /// Table in toml
        /// ===============================================
        [DllImport(dllName)]
        public static extern bool table_is_empty(IntPtr ptr);

        [DllImport(dllName)]
        public static extern uint table_len(IntPtr ptr);

        // return Item ptr
        [DllImport(dllName)]
        public static extern IntPtr table_get(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr table_get_keys(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr table_get_array_keys(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr table_get_inline_table_keys(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool table_contains_key(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport(dllName)]
        public static extern bool table_contains_table(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport(dllName)]
        public static extern bool table_contains_value(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport(dllName)]
        public static extern bool table_contains_array_of_tables(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport(dllName)]
        public static extern void table_dispose(IntPtr ptr);

        // return Table ptr
        [DllImport(dllName)]
        public static extern IntPtr table_new();

        // item_ptr: Item ptr
        [DllImport(dllName)]
        public static extern bool table_insert(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key, IntPtr item_ptr);

        [DllImport(dllName)]
        public static extern bool table_remove(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport(dllName)]
        public static extern void table_clear(IntPtr ptr);

        // retrun *const c_char
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr table_to_string(IntPtr ptr);

        /// ===============================================
        /// InlineTable in toml
        /// ===============================================
        [DllImport(dllName)]
        public static extern bool inline_table_is_empty(IntPtr ptr);

        [DllImport(dllName)]
        public static extern uint inline_table_len(IntPtr ptr);

        // return Value ptr
        [DllImport(dllName)]
        public static extern IntPtr inline_table_get(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr inline_table_get_keys(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr inline_table_get_array_keys(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr inline_table_get_inline_table_keys(IntPtr ptr);

        [DllImport(dllName)]
        public static extern bool inline_table_contains_key(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport(dllName)]
        public static extern void inline_table_dispose(IntPtr ptr);

        // return InlineTable ptr
        [DllImport(dllName)]
        public static extern IntPtr inline_table_new();

        // value_ptr: Value ptr
        [DllImport(dllName)]
        public static extern bool inline_table_insert(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key, IntPtr value_ptr);

        [DllImport(dllName)]
        public static extern bool inline_table_remove(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport(dllName)]
        public static extern void inline_table_clear(IntPtr ptr);

        // retrun *const c_char
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr inline_table_to_string(IntPtr ptr);

        /// ===============================================
        /// ArrayOfTables in toml
        /// ===============================================
        [DllImport(dllName)]
        public static extern bool table_array_is_empty(IntPtr ptr);

        [DllImport(dllName)]
        public static extern uint table_array_len(IntPtr ptr);

        // return Table ptr
        [DllImport(dllName)]
        public static extern IntPtr table_array_get(IntPtr ptr, uint index);

        [DllImport(dllName)]
        public static extern void table_array_dispose(IntPtr ptr);

        // return ArrayOfTables ptr
        [DllImport(dllName)]
        public static extern IntPtr table_array_new();

        // table_ptr: Table ptr
        [DllImport(dllName)]
        public static extern void table_array_push(IntPtr ptr, IntPtr table_ptr);

        [DllImport(dllName)]
        public static extern void table_array_remove(IntPtr ptr, uint index);

        [DllImport(dllName)]
        public static extern void table_array_clear(IntPtr ptr);

        // return Array ptr
        [DllImport(dllName)]
        public static extern IntPtr table_array_to_array(IntPtr ptr);

        // retrun *const c_char
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr table_array_to_string(IntPtr ptr);

        // ===============================================
        // String array in Rust
        // ===============================================
        [DllImport(dllName)]
        public static extern uint strs_len(IntPtr ptr);

        // return c_char ptr
        [DllImport(dllName)]
        public static extern IntPtr strs_get(IntPtr ptr, uint index);


        [DllImport(dllName)]
        public static extern void strs_dispose(IntPtr ptr);

        // ptr: c_char ptr
        [DllImport(dllName)]
        public static extern void str_dispose(IntPtr ptr);

        // parse toml test
        public static void ParseTomlTest()
        {
            Console.WriteLine("[rtoml]");
            // parse toml version
            Console.WriteLine("  - parse toml version:");
            Console.WriteLine("      version: " + get_version());
            Console.WriteLine("");

            // parse toml value
            Console.WriteLine("  - parse toml value:");
            // System.IntPtr doc = parse_toml_file("../../../../example.toml");

            string context = System.IO.File.ReadAllText("../../../../example.toml");
            System.IntPtr doc = document_parse_content(context);

            {
                System.IntPtr item_str = document_get(doc, "str_val");
                IntPtr str_ptr = item_as_str(item_str);
                Console.WriteLine("      str val:    " + Ptr2String(str_ptr));

                System.IntPtr item_int32 = document_get(doc, "int32_val");
                int int32_val = item_as_int32(item_int32);
                Console.WriteLine("      int32 val:  " + int32_val);

                System.IntPtr item_int64 = document_get(doc, "int64_val");
                long int64_val = item_as_int64(item_int64);
                Console.WriteLine("      int64 val:  " + int64_val);

                System.IntPtr item_float = document_get(doc, "float_val");
                float float_val = item_as_float(item_float);
                Console.WriteLine("      float val:  " + float_val);

                System.IntPtr item_double = document_get(doc, "double_val");
                double double_val = item_as_double(item_double);
                Console.WriteLine("      double val: " + double_val);
                Console.WriteLine("");
            }

            // get document keys
            Console.WriteLine("  - get document keys:");
            // get doc all keys
            System.IntPtr doc_keys_ptr = document_get_keys(doc);
            string[] doc_keys = Ptr2StringList(doc_keys_ptr);
            Console.WriteLine("      all keys(" + doc_keys.Length + "): ");
            foreach (string key in doc_keys)
            {
                Console.WriteLine("        " + key);
            }
            Console.WriteLine("");

            // get doc array keys
            System.IntPtr doc_array_keys_ptr = document_get_array_keys(doc);
            string[] doc_array_keys = Ptr2StringList(doc_array_keys_ptr);
            Console.WriteLine("      array keys(" + doc_array_keys.Length + "): ");
            foreach (string key in doc_array_keys)
            {
                Console.WriteLine("        " + key);
            }
            Console.WriteLine("");

            // get doc table keys
            System.IntPtr doc_table_keys_ptr = document_get_table_keys(doc);
            string[] doc_table_keys = Ptr2StringList(doc_table_keys_ptr);
            Console.WriteLine("      table keys(" + doc_table_keys.Length + "): ");
            foreach (string key in doc_table_keys)
            {
                Console.WriteLine("        " + key);
            }
            Console.WriteLine("");

            // get doc inline table keys
            System.IntPtr doc_inline_table_keys_ptr = document_get_inline_table_keys(doc);
            string[] doc_inline_table_keys = Ptr2StringList(doc_inline_table_keys_ptr);
            Console.WriteLine("      inline table keys(" + doc_inline_table_keys.Length + "): ");
            foreach (string key in doc_inline_table_keys)
            {
                Console.WriteLine("        " + key);
            }
            Console.WriteLine("");

            // get doc table array keys
            System.IntPtr doc_table_array_keys_ptr = document_get_table_array_keys(doc);
            string[] doc_table_array_keys = Ptr2StringList(doc_table_array_keys_ptr);
            Console.WriteLine("      table array keys(" + doc_table_array_keys.Length + "): ");
            foreach (string key in doc_table_array_keys)
            {
                Console.WriteLine("        " + key);
            }
            Console.WriteLine("");

            // parse table
            Console.WriteLine("  - parse table:");
            Console.WriteLine("      name: bundles");
            System.IntPtr bundles_item = document_get(doc, "bundles");
            System.IntPtr bundles_table = item_as_table(bundles_item);

            // get table all keys
            System.IntPtr table_keys_ptr = table_get_keys(bundles_table);
            string[] table_keys = Ptr2StringList(table_keys_ptr);
            Console.WriteLine("      all keys(" + table_keys.Length + "): ");
            foreach (string key in table_keys)
            {
                Console.WriteLine("        " + key);
            }
            Console.WriteLine("");

            // get table array keys
            System.IntPtr table_array_keys_ptr = table_get_array_keys(bundles_table);
            string[] table_array_keys = Ptr2StringList(table_array_keys_ptr);
            Console.WriteLine("      array keys(" + table_array_keys.Length + "): ");
            foreach (string key in table_array_keys)
            {
                Console.WriteLine("        " + key);
            }
            Console.WriteLine("");

            // get inline table keys
            System.IntPtr table_inline_table_keys_ptr = table_get_inline_table_keys(bundles_table);
            string[] table_inline_table_keys = Ptr2StringList(table_inline_table_keys_ptr);
            Console.WriteLine("      inline table keys(" + table_inline_table_keys.Length + "): ");
            foreach (string key in table_inline_table_keys)
            {
                Console.WriteLine("        " + key);
            }
            Console.WriteLine("");

            System.IntPtr includes_item = table_get(bundles_table, "includes");
            System.IntPtr includes_array = item_as_array(includes_item);
            uint includes_len = array_len(includes_array);
            Console.WriteLine("      includes(" + includes_len + "): ");
            List<string> includes = new List<string>();
            for (uint i = 0; i < includes_len; i++)
            {
                System.IntPtr val_ptr = array_get(includes_array, i);
                string str = Ptr2String(value_as_str(val_ptr));
                includes.Add(str);

                Console.WriteLine("        " + str);
            }
            Console.WriteLine("");


            System.IntPtr ignores_item = table_get(bundles_table, "ignores");
            System.IntPtr ignores_array = item_as_array(ignores_item);
            uint ignores_len = array_len(ignores_array);
            Console.WriteLine("      ignores(" + ignores_len + "): ");
            List<string> ignores = new List<string>();
            for (uint i = 0; i < ignores_len; i++)
            {
                System.IntPtr val_ptr = array_get(ignores_array, i);
                string str = Ptr2String(value_as_str(val_ptr));
                ignores.Add(str);

                Console.WriteLine("        " + str);
            }

            Console.WriteLine("");

            // parse inline table
            Console.WriteLine("  - parse inline table:");
            Console.WriteLine("      name: person");
            System.IntPtr person_item = document_get(doc, "person");
            System.IntPtr person_inline_table = item_as_inline_table(person_item);
            bool a = item_is_inline_table(person_item);


            // get table all keys
            System.IntPtr inline_table_keys_ptr = inline_table_get_keys(person_inline_table);
            string[] inline_table_keys = Ptr2StringList(inline_table_keys_ptr);
            Console.WriteLine("      all keys(" + inline_table_keys.Length + "): ");
            foreach (string key in inline_table_keys)
            {
                Console.WriteLine("        " + key);
            }
            Console.WriteLine("");

            // get table array keys
            System.IntPtr inline_table_array_keys_ptr = inline_table_get_array_keys(person_inline_table);
            string[] inline_table_array_keys = Ptr2StringList(inline_table_array_keys_ptr);
            Console.WriteLine("      array keys(" + inline_table_array_keys.Length + "): ");
            foreach (string key in inline_table_array_keys)
            {
                Console.WriteLine("        " + key);
            }
            Console.WriteLine("");

            // get inline table keys
            System.IntPtr inline_table_inline_table_keys_ptr = inline_table_get_inline_table_keys(person_inline_table);
            string[] inline_table_inline_table_keys = Ptr2StringList(inline_table_inline_table_keys_ptr);
            Console.WriteLine("      inline table keys(" + inline_table_inline_table_keys.Length + "): ");
            foreach (string key in inline_table_inline_table_keys)
            {
                Console.WriteLine("        " + key);
            }
            Console.WriteLine("  end parse");

            document_dispose(doc);

            GC.Collect();
            GC.WaitForPendingFinalizers();
        }

        // parse toml test
        public static void WriteTomlTest()
        {
            // parse toml value
            Console.WriteLine("  - write toml value:");

            string context = System.IO.File.ReadAllText("../../../../example.toml");
            IntPtr doc = document_parse_content(context);

            // change doc
            {
                document_insert(doc, "str_val", item_from_str("funnyland"));
            }

            // change inline table
            {
                IntPtr item_ptr = document_get(doc, "person");
                IntPtr inline_table_ptr = item_as_inline_table(item_ptr);
                inline_table_clear(inline_table_ptr);
                inline_table_insert(inline_table_ptr, "name", value_from_str("john"));
                inline_table_insert(inline_table_ptr, "age", value_from_i32(18));
                inline_table_insert(inline_table_ptr, "weight", value_from_float(65.5f));
            }

            // change table
            {
                IntPtr item_ptr = document_get(doc, "bundles");
                IntPtr table_ptr = item_as_table(item_ptr);
                table_remove(table_ptr, "list");
                table_insert(table_ptr, "patterns", item_from_array(array_new()));
            }

            // change array
            {
                IntPtr item_ptr = document_get(doc, "list");
                IntPtr array_ptr = item_as_array(item_ptr);
                for (int i = 3; i < 10; i++)
                {
                    IntPtr inline_table_ptr = inline_table_new();
                    inline_table_insert(inline_table_ptr, "foo", value_from_i32(i));
                    array_push(array_ptr, inline_table_ptr);
                    inline_table_dispose(inline_table_ptr);
                }
                array_pretty(array_ptr);


            }

            // change array of tables
            {
                IntPtr item_ptr = document_get(doc, "fruits");
                IntPtr table_array_ptr = item_as_array_of_tables(item_ptr);
                IntPtr new_table_ptr = table_new();
                table_insert(new_table_ptr, "name", item_from_str("orange"));
                table_array_push(table_array_ptr, new_table_ptr);
                table_dispose(new_table_ptr);
            }

            // 中文字符串处理
            IntPtr ptr = document_to_string(doc);
            string res = Marshal.PtrToStringUTF8(ptr);

            Console.WriteLine(res);
            File.WriteAllText("../../../../new_example.toml",res);
            document_dispose(doc);

            GC.Collect();
            GC.WaitForPendingFinalizers();
        }

        private static string Ptr2String(IntPtr ptr)
        {
            string str = Marshal.PtrToStringUTF8(ptr);
            str_dispose(ptr);
            return str;
        }

        private static string[] Ptr2StringList(IntPtr strs_ptr)
        {
            if (strs_ptr != System.IntPtr.Zero)
            {
                UInt32 len = strs_len(strs_ptr);
                if (len != 0)
                {
                    string[] files = new string[len];
                    for (uint i = 0; i < len; i++)
                    {
                        IntPtr ptr = strs_get(strs_ptr, i);
                        files[i] = Ptr2String(ptr);
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
