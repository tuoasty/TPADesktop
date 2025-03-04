import {NewImage} from "@/ types/image.ts";

const getMimeType = (fileName: string): string => {
    const extension = fileName.split('.').pop()?.toLowerCase();
    switch (extension) {
        case 'jpg':
        case 'jpeg':
            return 'image/jpeg';
        case 'png':
            return 'image/png';
        case 'gif':
            return 'image/gif';
        case 'webp':
            return 'image/webp';
        default:
            return 'application/octet-stream';
    }
};

export async function createImageFile(e: React.ChangeEvent<HTMLInputElement>){
    const file = e.target.files?.[0];
    if (!file) return;

    const arrayBuffer = await file.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);

    const base64String = btoa(
        Array.from(uint8Array)
            .map(byte => String.fromCharCode(byte))
            .join('')
    );

    const image:NewImage = {
        imageData:base64String,
        mimeType:getMimeType(file.name),
        fileName:file.name,
    };

    return image;
}