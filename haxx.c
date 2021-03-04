#include <spa/param/video/format-utils.h>
#include <spa/debug/types.h>
#include <spa/param/video/type-info.h>

static char params_buffer[1024] = { 0 };

extern const struct spa_pod * build_video_params() {

    struct spa_pod_builder pod_builder;

    pod_builder = SPA_POD_BUILDER_INIT (params_buffer, sizeof (params_buffer));
    return spa_pod_builder_add_object (
        &pod_builder,
        SPA_TYPE_OBJECT_Format, SPA_PARAM_EnumFormat,
        SPA_FORMAT_mediaType, SPA_POD_Id (SPA_MEDIA_TYPE_video),
        SPA_FORMAT_mediaSubtype, SPA_POD_Id (SPA_MEDIA_SUBTYPE_raw),
        SPA_FORMAT_VIDEO_format, SPA_POD_CHOICE_ENUM_Id (4,
                                                        SPA_VIDEO_FORMAT_RGBA,
                                                        SPA_VIDEO_FORMAT_RGBx,
                                                        SPA_VIDEO_FORMAT_BGRx,
                                                        SPA_VIDEO_FORMAT_BGRA),
        SPA_FORMAT_VIDEO_size, SPA_POD_CHOICE_RANGE_Rectangle (&SPA_RECTANGLE (320, 240),
                                                            &SPA_RECTANGLE (1, 1),
                                                            &SPA_RECTANGLE (4096, 4096)),
        SPA_FORMAT_VIDEO_framerate, SPA_POD_CHOICE_RANGE_Fraction (&SPA_FRACTION (60, 1),
                                                                &SPA_FRACTION (0, 1),
                                                                &SPA_FRACTION (144, 1)));
}