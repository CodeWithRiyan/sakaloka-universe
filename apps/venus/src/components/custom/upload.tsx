import type { ImageMetadata } from "@/types"
import { Upload, X } from "lucide-react"
import React, { useCallback } from "react"
import { useDropzone } from "react-dropzone"
import { toast } from "sonner"
import { Button } from "../ui/button"
import { Card, CardContent } from "../ui/card"

export interface FileWithPreview extends File {
  preview?: string
}

interface UploadDNDProps
  extends Omit<React.HTMLAttributes<HTMLDivElement>, "onChange"> {
  value?: FileWithPreview | null
  imageUrl?: string
  imageMetadata?: ImageMetadata
  onDeleteImageUrl?: (value: boolean) => void
  onChange?: (file: FileWithPreview | null) => void
  isLoading?: boolean
  accept?: string[]
  maxSize?: number
  placeholder?: {
    drag?: string
    click?: string
    loading?: string
  }
  name?: string
  disabled?: boolean
  // Form-related props that will be passed when used with FormControl
  id?: string
  "aria-invalid"?: boolean
}

export function UploadDND({
  value,
  imageMetadata,
  onDeleteImageUrl,
  onChange,
  isLoading = false,
  accept = [".jpeg", ".jpg", ".png", ".gif", ".webp"],
  maxSize = 5 * 1024 * 1024, // 5MB
  placeholder = {},
  className,
  disabled = false,
  "aria-invalid": ariaInvalid,
  ...props
}: UploadDNDProps) {
  const [url, setUrl] = React.useState(imageMetadata?.url)
  const {
    drag = "Lepaskan file di sini",
    click = "Drag & drop gambar atau klik untuk memilih",
    loading = "Mengupload...",
  } = placeholder

  // Determine current file value
  const currentFile = value ?? null

  // Field properties
  const isFieldDisabled = disabled || isLoading

  const handleFileChange = useCallback(
    (file: FileWithPreview | null) => {
      if (onChange) {
        onChange(file)
        if (file) onDeleteImageUrl?.(false)
      }
    },
    [onChange, onDeleteImageUrl]
  )

  const onDrop = useCallback(
    (acceptedFiles: File[]) => {
      const file = acceptedFiles[0]
      if (file) {
        const fileWithPreview = Object.assign(file, {
          preview: URL.createObjectURL(file),
        }) as FileWithPreview

        handleFileChange(fileWithPreview)
      }
    },
    [handleFileChange]
  )

  const onDropRejected = useCallback(
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (rejectedFiles: any[]) => {
      const rejection = rejectedFiles[0]
      if (rejection) {
        const error = rejection.errors[0]
        let errorMsg = "File tidak valid"

        if (error.code === "file-too-large") {
          errorMsg = `File terlalu besar. Maksimal ${(
            maxSize /
            1024 /
            1024
          ).toFixed(0)}MB`
        } else if (error.code === "file-invalid-type") {
          errorMsg = "Format file tidak didukung"
        }

        toast.error(errorMsg)
        handleFileChange(null)
      }
    },
    [handleFileChange, maxSize]
  )

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    onDropRejected,
    accept: {
      "image/*": accept,
    },
    maxFiles: 1,
    maxSize,
    disabled: isFieldDisabled,
  })

  const handleRemoveFile = useCallback(() => {
    if (currentFile?.preview) {
      URL.revokeObjectURL(currentFile.preview)
    }
    setUrl(undefined)
    onDeleteImageUrl?.(true)
    handleFileChange(null)
  }, [currentFile?.preview, handleFileChange, onDeleteImageUrl])

  // Determine error state
  const hasError = ariaInvalid
  const errorText = hasError ? "File tidak valid" : undefined

  // Determine file type for accept types display
  const getAcceptTypesDisplay = () => {
    const extensions = accept.map((ext) => ext.replace(".", "").toUpperCase())
    return extensions.join(", ")
  }

  return (
    <div className={`space-y-2 ${className || ""}`} {...props}>
      <div className="space-y-2">
        {currentFile || url ? (
          <Card
            className={`w-full rounded-lg border shadow-xs ${
              errorText ? "border-red-300" : "border-gray-200"
            }`}
          >
            <CardContent className="relative p-4">
              <div className="flex items-center space-x-4">
                <div className="relative size-20 flex-shrink-0 overflow-hidden rounded-lg bg-gray-100">
                  <img
                    src={currentFile?.preview || url || ""}
                    alt="Preview"
                    className="h-full w-full object-cover"
                    onLoad={() => {
                      // Clean up object URL after image loads
                      if (currentFile?.preview) {
                        URL.revokeObjectURL(currentFile.preview)
                      }
                    }}
                  />
                </div>
                <div className="min-w-0 flex-1 pr-12">
                  <div
                    className="line-clamp-2 text-sm leading-tight font-medium break-all text-gray-900"
                    title={currentFile?.name || imageMetadata?.filename}
                  >
                    {currentFile?.name || imageMetadata?.filename}
                  </div>
                  <p className="mt-2 text-xs text-gray-500">
                    {(
                      (currentFile?.size || imageMetadata?.size || 0) /
                      1024 /
                      1024
                    ).toFixed(2)}{" "}
                    MB
                  </p>
                </div>
              </div>
              <Button
                type="button"
                variant="ghost"
                size="sm"
                onClick={handleRemoveFile}
                disabled={isFieldDisabled}
                className="absolute top-2 right-2 h-8 w-8 p-0 hover:bg-red-50 hover:text-red-600"
                title="Hapus file"
              >
                <X className="h-4 w-4" />
              </Button>
            </CardContent>
          </Card>
        ) : (
          <div
            {...getRootProps()}
            className={`cursor-pointer rounded-lg border-2 border-dashed p-4 text-center transition-colors sm:p-6 ${
              isDragActive
                ? "border-primary bg-primary/5"
                : errorText
                  ? "border-red-300 bg-red-50/50"
                  : "border-gray-300 hover:border-gray-400"
            } ${isFieldDisabled ? "cursor-not-allowed opacity-50" : ""}`}
          >
            <input {...getInputProps()} />
            <div className="flex flex-col items-center space-y-2">
              <Upload
                className={`h-8 w-8 sm:h-10 sm:w-10 ${
                  errorText ? "text-red-400" : "text-gray-400"
                }`}
              />
              <div className="space-y-1">
                <p
                  className={`text-xs font-medium sm:text-sm ${
                    errorText ? "text-red-600" : "text-gray-600"
                  }`}
                >
                  {isLoading ? loading : isDragActive ? drag : click}
                </p>
                <p className="text-xs text-gray-500">
                  {getAcceptTypesDisplay()} hingga{" "}
                  {(maxSize / 1024 / 1024).toFixed(0)}MB
                </p>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

export default UploadDND
