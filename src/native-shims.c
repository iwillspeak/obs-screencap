#include <spa/debug/types.h>
#include <spa/param/video/format-utils.h>
#include <spa/param/video/type-info.h>

static char params_buffer[1024] = {0};

extern const struct spa_pod *build_video_params() {

  struct spa_pod_builder pod_builder;

  pod_builder = SPA_POD_BUILDER_INIT(params_buffer, sizeof(params_buffer));
  return spa_pod_builder_add_object(
      &pod_builder, SPA_TYPE_OBJECT_Format, SPA_PARAM_EnumFormat,
      SPA_FORMAT_mediaType, SPA_POD_Id(SPA_MEDIA_TYPE_video),
      SPA_FORMAT_mediaSubtype, SPA_POD_Id(SPA_MEDIA_SUBTYPE_raw),
      SPA_FORMAT_VIDEO_format,
      SPA_POD_CHOICE_ENUM_Id(4, SPA_VIDEO_FORMAT_RGBA, SPA_VIDEO_FORMAT_RGBx,
                             SPA_VIDEO_FORMAT_BGRx, SPA_VIDEO_FORMAT_BGRA),
      SPA_FORMAT_VIDEO_size,
      SPA_POD_CHOICE_RANGE_Rectangle(&SPA_RECTANGLE(1920, 1080),
                                     &SPA_RECTANGLE(1, 1),
                                     &SPA_RECTANGLE(4096, 4096)),
      SPA_FORMAT_VIDEO_framerate,
      SPA_POD_CHOICE_RANGE_Fraction(&SPA_FRACTION(60, 1), &SPA_FRACTION(0, 1),
                                    &SPA_FRACTION(144, 1)));
}

extern const struct spa_pod *build_stream_param() {

  struct spa_pod_builder pod_builder;

  pod_builder = SPA_POD_BUILDER_INIT(params_buffer, sizeof(params_buffer));
  return spa_pod_builder_add_object(
      &pod_builder, SPA_TYPE_OBJECT_ParamBuffers, SPA_PARAM_Buffers,
      SPA_PARAM_BUFFERS_dataType,
      SPA_POD_Int((1 << SPA_DATA_MemPtr) | (1 << SPA_DATA_DmaBuf)));
}

extern const int spa_format_parse_rs(const struct spa_pod *format,
                                     uint32_t *media_type,
                                     uint32_t *media_subtype) {
  return spa_format_parse(format, media_type, media_subtype);
}

extern const int
spa_format_video_raw_parse_rs(const struct spa_pod *format,
                              struct spa_video_info_raw *info) {
  return spa_format_video_raw_parse(format, info);
}
