using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace csharp_link_rust.libs
{
    public class rxml
    {
        #if !UNITY_EDITOR && UNITY_IPHONE
            const string dllName = "__Internal";
        #else
            const string dllName = "../../../../../target/debug/rxml";
        #endif

        // ===============================================
        // Info
        // ===============================================
        // return c_char ptr
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr get_version();

        // ===============================================
        // Document
        // ===============================================
        // return Document ptr
        [DllImport(dllName)]
        public static extern IntPtr document_parse_file([MarshalAs(UnmanagedType.LPUTF8Str)] string path);

        // return Document ptr
        [DllImport(dllName)]
        public static extern IntPtr document_parse_content([MarshalAs(UnmanagedType.LPUTF8Str)] string content);

        [DllImport(dllName)]
        public static extern void document_dispose(IntPtr ptr);

        // return c_char ptr
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr document_get_version(IntPtr ptr);

        // return c_char ptr
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr document_get_encoding(IntPtr ptr);

        [DllImport(dllName)]
        public static extern Int32 document_get_children_len(IntPtr ptr);

        // return Element ptr
        [DllImport(dllName)]
        public static extern IntPtr document_get_child(IntPtr ptr, UInt32 index);

        // return c_char ptr
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr document_tree_text(IntPtr ptr);

        // ===============================================
        // Element
        // ===============================================
        [DllImport(dllName)]
        public static extern void element_dispose(IntPtr ptr);

        // return c_char ptr
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr element_get_name(IntPtr ptr);

        // return c_char ptr
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr element_get_text(IntPtr ptr);

        // return Vec<String> ptr
        [DllImport(dllName)]
        public static extern IntPtr element_get_attribute_keys(IntPtr ptr);

        // return c_char ptr
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr element_get_attribute_value(IntPtr ptr, [MarshalAs(UnmanagedType.LPUTF8Str)] string key);

        [DllImport(dllName)]
        public static extern Int32 element_get_children_len(IntPtr ptr);

        // return Element ptr
        [DllImport(dllName)]
        public static extern IntPtr element_get_child(IntPtr ptr, UInt32 index);

        // ============================================================
        // Vec<String> | c_char | error
        // ============================================================
        [DllImport(dllName)]
        public static extern uint strs_len(IntPtr ptr);

        // return c_char ptr
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr strs_get(IntPtr ptr, uint index);

        [DllImport(dllName)]
        public static extern void strs_dispose(IntPtr ptr);

        // ptr: c_char ptr
        [DllImport(dllName)]
        public static extern void str_dispose(IntPtr ptr);

        // return c_char ptr
        // convert to string by using Marshal.PtrToStringUTF8(ptr)
        [DllImport(dllName)]
        public static extern IntPtr try_get_err();

        public static void ParseXmlTest()
        {
            Console.WriteLine("  - parse xml value:");

            // 解析文件
            IntPtr doc_ptr = document_parse_file("../../../../example.xml");

            // 树形结构文本
            string tree_text = Ptr2String(document_tree_text(doc_ptr));
            Console.WriteLine(tree_text);
            Console.WriteLine();

            // 获取遍历 root 结点
            for (UInt32 i = 0; i< document_get_children_len(doc_ptr); i++)
            {
                // 获取 element
                IntPtr element_ptr = document_get_child(doc_ptr, i);
                {
                    // 获取 element 名称
                    string name = Ptr2String(element_get_name(element_ptr));
                    Console.WriteLine(name);

                    // 获取 element 的所有 key 和 value
                    IntPtr keys_ptr = element_get_attribute_keys(element_ptr);
                    foreach (var key in Ptr2StringList(keys_ptr))
                    {
                        IntPtr val_ptr = element_get_attribute_value(element_ptr, key);
                        Console.WriteLine("  " + key + ": " + Ptr2String(val_ptr));
                    }
                }

                // 获取子 element
                for (UInt32 j = 0; j < element_get_children_len(element_ptr); j++)
                {
                    // 获取 element
                    IntPtr child_element_ptr = element_get_child(element_ptr, i);

                    // 获取 element 名称
                    string name = Ptr2String(element_get_name(child_element_ptr));
                    Console.WriteLine("  " + name);

                    // 获取 element 的所有 key 和 value
                    IntPtr keys_ptr = element_get_attribute_keys(child_element_ptr);
                    foreach (var key in Ptr2StringList(keys_ptr))
                    {
                        IntPtr val_ptr = element_get_attribute_value(child_element_ptr, key);
                        Console.WriteLine("    " + key + ": " + Ptr2String(val_ptr));
                    }
                }
            }

            document_dispose(doc_ptr);
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
