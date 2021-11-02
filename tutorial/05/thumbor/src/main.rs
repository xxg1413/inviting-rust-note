/**
 *  图片服务器：先做resize,再添加水印，再使用一个滤镜
 */


 /* 图片操作类 */
struct ImageSpec {

    specs: Vec<Spec>,
}


/* 图片操作行为 */
 enum Spec {
     Resize(Resize),
     Crop(Crop),
 }

 /* 添加水印 */
struct Crop {

}


/* 缩小尺寸 */
 struct Resize {
     
    width: u32,
    height: u32,
 }

