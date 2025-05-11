
IMGID="03"
POSITION="1024x820+1024+412"
ASSET_NAME="screen_"

convert Scifi_Panels_${IMGID}_ambientOcclusion.png -crop $POSITION ../../assets/textures/${ASSET_NAME}ambientocclusion.png
convert Scifi_Panels_${IMGID}_basecolor.png -crop $POSITION ../../assets/textures/${ASSET_NAME}basecolor.png
convert Scifi_Panels_${IMGID}_emissive.png -crop $POSITION ../../assets/textures/${ASSET_NAME}emissive.png
convert Scifi_Panels_${IMGID}_roughness.png -crop $POSITION ../../assets/textures/${ASSET_NAME}roughness.png
convert Scifi_Panels_${IMGID}_normal.png -crop $POSITION ../../assets/textures/${ASSET_NAME}normal.png

# those are unused in the project:

# convert Scifi_Panels_${IMGID}_emissiveMask.png -crop $POSITION ../../assets/textures/${ASSET_NAME}emissiveMask.png
# convert Scifi_Panels_${IMGID}_height.png -crop $POSITION ../../assets/textures/${ASSET_NAME}height.png
